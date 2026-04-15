package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SyncDaemonPayload struct {
	ID      string `json:"id"`
	Status  string `json:"status"`
	Payload string `json:"payload"`
}

type HybridMCPRAGDaemon struct {
	dbWrapper   *db.DB
	ticker      *time.Ticker
	quit        chan struct{}
	cloudAPIURL string
}

func NewHybridMCPRAGDaemon(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *HybridMCPRAGDaemon {
	if cloudAPIURL == "" {
		cloudAPIURL = os.Getenv("OHC_CORE_URL")
	}
	if cloudAPIURL == "" {
		cloudAPIURL = "http://localhost:8080"
	}

	return &HybridMCPRAGDaemon{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
	}
}

func (d *HybridMCPRAGDaemon) Start(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		// Only run in standalone/SQLite mode
		slog.Debug("sync_daemon: HybridMCPRAGDaemon disabled (not in standalone SQLite mode)")
		return
	}

	go func() {
		for {
			select {
			case <-d.quit:
				d.ticker.Stop()
				return
			case <-ctx.Done():
				d.ticker.Stop()
				return
			default:
				processed := d.ProcessSync(ctx)
				if !processed {
					time.Sleep(1 * time.Second)
				}
			}
		}
	}()
}

func (d *HybridMCPRAGDaemon) Stop() {
	close(d.quit)
}

func (d *HybridMCPRAGDaemon) ProcessSync(ctx context.Context) bool {
	if !d.dbWrapper.IsSQLite() {
		return false
	}
	start := time.Now()

	tx, err := d.dbWrapper.Begin(ctx)
	if err != nil {
		slog.Error("sync_daemon: failed to begin transaction", "error", err)
		return false
	}
	defer tx.Rollback(ctx)

		query := "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false AND status IN ('PENDING', 'BURSTING') LIMIT 500"
	if d.dbWrapper.IsSQLite() {
		query = "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = 0 AND status IN ('PENDING', 'BURSTING') LIMIT 500"
	}
	rows, err := tx.Query(ctx, query)
	if err != nil {
		slog.Error("sync_daemon: failed to query agent_missions", "error", err)
		return false
	}
	defer rows.Close()

	var payloads []SyncDaemonPayload
	var ids []string

	for rows.Next() {
		var id, status, payloadData string
		if err := rows.Scan(&id, &status, &payloadData); err != nil {
			slog.Error("sync_daemon: failed to scan agent_missions", "error", err)
			continue
		}

		// Sanitize payload data for PI and private markers
		var parsedPayload map[string]interface{}
		if err := json.Unmarshal([]byte(payloadData), &parsedPayload); err == nil {
			parsedIface := SanitizePayloadMap(parsedPayload)
			if redactedBytes, err := json.Marshal(parsedIface); err == nil {
				payloadData = string(redactedBytes)
			}
		} else {
			sanitizedStr, _ := SanitizePayload(payloadData)
			payloadData = sanitizedStr
		}

		payloads = append(payloads, SyncDaemonPayload{
			ID:      id,
			Status:  status,
			Payload: payloadData,
		})
		ids = append(ids, id)
	}

	if len(payloads) == 0 {
		return false
	}

	if err := d.sendToCloud(ctx, payloads); err != nil {
		slog.Error("sync_daemon: failed to send agent_missions to cloud", "error", err)
		return false
	}

	// Mark as synced
	if len(ids) > 0 {
		idList := ""
		for i, id := range ids {
			if i > 0 {
				idList += ","
			}
			idList += fmt.Sprintf("'%s'", id)
		}
		updateQuery := fmt.Sprintf("UPDATE agent_missions SET synced_to_cloud = true WHERE id IN (%s)", idList)
		if d.dbWrapper.IsSQLite() {
			updateQuery = fmt.Sprintf("UPDATE agent_missions SET synced_to_cloud = 1 WHERE id IN (%s)", idList)
		}
		_, err := tx.Exec(ctx, updateQuery)
		if err != nil {
			slog.Error("sync_daemon: failed to update agent_missions status in bulk", "error", err)
			return false
		} else {
			telemetry.RecordSyncEscalation(ctx, int64(len(ids)))
		}
	}

	if err := tx.Commit(ctx); err != nil {
		slog.Error("sync_daemon: failed to commit transaction", "error", err)
		return false
	}

	telemetry.RecordSyncDaemonBatchSize(ctx, int64(len(payloads)))

	telemetry.RecordSyncLatency(ctx, float64(time.Since(start).Milliseconds()))

	slog.Debug("sync_daemon: successfully synced agent_missions", "count", len(payloads))
	return true
}

func (d *HybridMCPRAGDaemon) sendToCloud(ctx context.Context, payloads []SyncDaemonPayload) error {
	jsonData, err := json.Marshal(payloads)
	if err != nil {
		return fmt.Errorf("marshal payloads: %w", err)
	}
	telemetry.RecordSyncPayloadSize(ctx, int64(len(jsonData)))

	syncEndpoint := fmt.Sprintf("%s/api/sync/missions", d.cloudAPIURL)

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("do request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("unexpected status %d: %s", resp.StatusCode, string(body))
	}

	return nil
}
