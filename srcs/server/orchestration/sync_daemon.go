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
		slog.Info("sync_daemon: HybridMCPRAGDaemon disabled (not in standalone SQLite mode)")
		return
	}

	go func() {
		for {
			select {
			case <-d.ticker.C:
				d.ProcessSync(ctx)
			case <-d.quit:
				d.ticker.Stop()
				return
			case <-ctx.Done():
				d.ticker.Stop()
				return
			}
		}
	}()
}

func (d *HybridMCPRAGDaemon) Stop() {
	close(d.quit)
}

func (d *HybridMCPRAGDaemon) ProcessSync(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		return
	}

	syncedLiteral := "false"
	if d.dbWrapper.IsSQLite() {
		syncedLiteral = "0"
	}

	rows, err := d.dbWrapper.Query(ctx, fmt.Sprintf("SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = %s AND status = 'CLOUD_ESCALATION' LIMIT 100", syncedLiteral))
	if err != nil {
		slog.Error("sync_daemon: failed to query agent_missions", "error", err)
		return
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
		var sanitizeRecursively func(data interface{}) interface{}
		sanitizeRecursively = func(data interface{}) interface{} {
			switch v := data.(type) {
			case string:
				sanitized, _ := SanitizePayload(v)
				return sanitized
			case map[string]interface{}:
				for key, val := range v {
					v[key] = sanitizeRecursively(val)
				}
				return v
			case []interface{}:
				for i, val := range v {
					v[i] = sanitizeRecursively(val)
				}
				return v
			default:
				return v
			}
		}

		var parsedPayload map[string]interface{}
		if err := json.Unmarshal([]byte(payloadData), &parsedPayload); err == nil {
			parsedIface := sanitizeRecursively(parsedPayload)
			if redactedBytes, err := json.Marshal(parsedIface); err == nil {
				payloadData = string(redactedBytes)
			}
		} else {
			payloadData, _ = SanitizePayload(payloadData)
		}

		payloads = append(payloads, SyncDaemonPayload{
			ID:      id,
			Status:  status,
			Payload: payloadData,
		})
		ids = append(ids, id)
	}

	if len(payloads) == 0 {
		return
	}

	if err := d.sendToCloud(ctx, payloads); err != nil {
		slog.Error("sync_daemon: failed to send agent_missions to cloud", "error", err)
		return
	}

	// Mark as synced
	if len(ids) > 0 {
		syncedLiteral := "true"
		if d.dbWrapper.IsSQLite() {
			syncedLiteral = "1"
		}

		// Use parameterized query for IN clause to prevent SQL injection
		placeholders := ""
		args := make([]interface{}, len(ids))
		for i, id := range ids {
			if i > 0 {
				placeholders += ","
			}
			placeholders += fmt.Sprintf("$%d", i+1)
			args[i] = id
		}

		query := fmt.Sprintf("UPDATE agent_missions SET synced_to_cloud = %s WHERE id IN (%s)", syncedLiteral, placeholders)
		_, err := d.dbWrapper.Exec(ctx, query, args...)
		if err != nil {
			slog.Error("sync_daemon: failed to update agent_missions status in bulk", "error", err)
		} else {
			telemetry.RecordSyncEscalation(ctx, int64(len(ids)))
		}
	}

	slog.Debug("sync_daemon: successfully synced agent_missions", "count", len(payloads))
}

func (d *HybridMCPRAGDaemon) sendToCloud(ctx context.Context, payloads []SyncDaemonPayload) error {
	jsonData, err := json.Marshal(payloads)
	if err != nil {
		return fmt.Errorf("marshal payloads: %w", err)
	}

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
