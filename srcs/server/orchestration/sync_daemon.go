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
	"sync"
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

		// Sanitize payload data
		sanitizedPayload, err := SanitizePayload(payloadData)
		if err != nil {
			slog.Error("sync_daemon: failed to sanitize payload", "error", err)
			continue
		}

		// Sanitize payload data for PII as well
		var sanitizeRecursively func(data interface{}) interface{}
		sanitizeRecursively = func(data interface{}) interface{} {
			switch v := data.(type) {
			case string:
				return telemetry.RedactPII(v)
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
		if err := json.Unmarshal([]byte(sanitizedPayload), &parsedPayload); err == nil {
			parsedIface := sanitizeRecursively(parsedPayload)
			if redactedBytes, err := json.Marshal(parsedIface); err == nil {
				sanitizedPayload = string(redactedBytes)
			}
		} else {
			sanitizedPayload = telemetry.RedactPII(sanitizedPayload)
		}

		payloads = append(payloads, SyncDaemonPayload{
			ID:      id,
			Status:  status,
			Payload: sanitizedPayload,
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

	// Mark as synced and record metrics
	if len(ids) > 0 {
		placeholders := ""
		args := make([]interface{}, len(ids))
		for i, id := range ids {
			if i > 0 {
				placeholders += ","
			}
			placeholders += fmt.Sprintf("$%d", i+1)
			args[i] = id
		}
		query := fmt.Sprintf("UPDATE agent_missions SET synced_to_cloud = true, status = 'PENDING_CLOUD' WHERE id IN (%s)", placeholders)
		_, err := d.dbWrapper.Exec(ctx, query, args...)
		if err != nil {
			slog.Error("sync_daemon: failed to update agent_missions status in bulk", "error", err)
		}

		if telemetry.SyncEscalationsCount != nil {
			telemetry.SyncEscalationsCount.Add(ctx, int64(len(ids)))
		}
	}

	slog.Info("sync_daemon: successfully synced agent_missions", "count", len(payloads))

	d.pollCloudResults(ctx)
}

func (d *HybridMCPRAGDaemon) pollCloudResults(ctx context.Context) {
	// Query local DB for missions that are waiting for cloud completion
	rows, err := d.dbWrapper.Query(ctx, "SELECT id FROM agent_missions WHERE status = 'PENDING_CLOUD' LIMIT 100")
	if err != nil {
		slog.Error("sync_daemon: failed to query local agent_missions for polling", "error", err)
		return
	}
	defer rows.Close()

	var pendingIDs []string
	for rows.Next() {
		var id string
		if err := rows.Scan(&id); err == nil {
			pendingIDs = append(pendingIDs, id)
		}
	}

	if len(pendingIDs) == 0 {
		return
	}

	// Poll the cloud for completion concurrently
	var wg sync.WaitGroup
	for _, id := range pendingIDs {
		wg.Add(1)
		go func(missionID string) {
			defer wg.Done()
			d.fetchResultFromCloud(ctx, missionID)
		}(id)
	}
	wg.Wait()
}

func (d *HybridMCPRAGDaemon) fetchResultFromCloud(ctx context.Context, missionID string) {
	fetchEndpoint := fmt.Sprintf("%s/api/sync/missions/%s", d.cloudAPIURL, missionID)

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, fetchEndpoint, nil)
	if err != nil {
		slog.Error("sync_daemon: failed to create fetch request", "error", err)
		return
	}

	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		slog.Error("sync_daemon: failed to fetch result from cloud", "error", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		// Cloud might not be done yet, skip
		return
	}

	var result SyncDaemonPayload
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		slog.Error("sync_daemon: failed to decode cloud result", "error", err)
		return
	}

	if result.Status == "DONE" {
		query := "UPDATE agent_missions SET status = 'DONE', payload = $1 WHERE id = $2"
		_, err := d.dbWrapper.Exec(ctx, query, result.Payload, result.ID)
		if err != nil {
			slog.Error("sync_daemon: failed to update local agent_missions with cloud result", "error", err)
		} else {
			slog.Info("sync_daemon: successfully pulled completed mission from cloud", "id", result.ID)
		}
	}
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
