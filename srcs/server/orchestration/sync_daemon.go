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
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
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
	syncCount   metric.Int64Counter
}

func NewHybridMCPRAGDaemon(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *HybridMCPRAGDaemon {
	if cloudAPIURL == "" {
		cloudAPIURL = os.Getenv("OHC_CORE_URL")
	}
	if cloudAPIURL == "" {
		cloudAPIURL = "http://localhost:8080"
	}

	meter := otel.Meter("orchestration")
	syncCount, err := meter.Int64Counter("ohc.sync.escalations.count", metric.WithDescription("Number of synced missions"))
	if err != nil {
		slog.Error("sync_daemon: failed to create metric ohc.sync.escalations.count", "error", err)
	}

	return &HybridMCPRAGDaemon{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
		syncCount:   syncCount,
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

	// 1. Sync from Local to Cloud
	d.syncToCloud(ctx)

	// 2. Sync results back from Cloud to Local
	d.syncFromCloud(ctx)
}

func (d *HybridMCPRAGDaemon) syncToCloud(ctx context.Context) {
	// Monitor local SQLite for missions with status = 'CLOUD_ESCALATION'
	rows, err := d.dbWrapper.Query(ctx, "SELECT id, status, payload FROM agent_missions WHERE status = 'CLOUD_ESCALATION' LIMIT 100")
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

		// Sanitize payload data for PI and private tags
		var sanitizeRecursively func(data interface{}) interface{}
		sanitizeRecursively = func(data interface{}) interface{} {
			switch v := data.(type) {
			case string:
				s, _ := SanitizePayload(v)
				return s
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

	// Mark as PENDING in local so it doesn't get synced again until done
	if len(ids) > 0 {
		idList := ""
		for i, id := range ids {
			if i > 0 {
				idList += ","
			}
			idList += fmt.Sprintf("'%s'", id)
		}
		query := fmt.Sprintf("UPDATE agent_missions SET status = 'CLOUD_PENDING' WHERE id IN (%s)", idList)
		_, err := d.dbWrapper.Exec(ctx, query)
		if err != nil {
			slog.Error("sync_daemon: failed to update agent_missions status in bulk", "error", err)
		}

		if d.syncCount != nil {
			d.syncCount.Add(ctx, int64(len(payloads)))
		}
	}

	slog.Info("sync_daemon: successfully synced agent_missions", "count", len(payloads))
}

func (d *HybridMCPRAGDaemon) syncFromCloud(ctx context.Context) {
	// Find missions in local DB that are waiting for cloud
	rows, err := d.dbWrapper.Query(ctx, "SELECT id FROM agent_missions WHERE status = 'CLOUD_PENDING' LIMIT 100")
	if err != nil {
		slog.Error("sync_daemon: failed to query local agent_missions", "error", err)
		return
	}
	defer rows.Close()

	var ids []string
	for rows.Next() {
		var id string
		if err := rows.Scan(&id); err != nil {
			continue
		}
		ids = append(ids, id)
	}

	if len(ids) == 0 {
		return
	}

	// Poll Cloud for these missions using API
	if err := d.fetchFromCloud(ctx, ids); err != nil {
		slog.Error("sync_daemon: failed to fetch agent_missions from cloud", "error", err)
	}
}

func (d *HybridMCPRAGDaemon) fetchFromCloud(ctx context.Context, ids []string) error {
	syncEndpoint := fmt.Sprintf("%s/api/sync/missions/poll", d.cloudAPIURL)

	reqBody, _ := json.Marshal(map[string][]string{"ids": ids})
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(reqBody))
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
		// API doesn't exist?
		if resp.StatusCode == 404 {
			// This might be missing on the server, gracefully handle
			return nil
		}
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("unexpected status %d: %s", resp.StatusCode, string(body))
	}

	var results []SyncDaemonPayload
	if err := json.NewDecoder(resp.Body).Decode(&results); err != nil {
		return fmt.Errorf("decode response: %w", err)
	}

	for _, result := range results {
		if result.Status == "DONE" {
			_, err = d.dbWrapper.Exec(ctx, "UPDATE agent_missions SET status = 'DONE', payload = $1 WHERE id = $2", result.Payload, result.ID)
			if err != nil {
				slog.Error("sync_daemon: failed to update local agent_missions", "error", err, "id", result.ID)
			}
		}
	}

	return nil
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
