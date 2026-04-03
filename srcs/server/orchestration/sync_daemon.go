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

// AutoDreamPayload defines the format for synchronized RAG items.
type AutoDreamPayload struct {
	Type     string `json:"type"` // "embedding" or "mission"
	ID       string `json:"id"`
	Data     string `json:"data"`
	Metadata string `json:"metadata"`
}

// HybridSyncDaemon bridges the standalone SQLite node to the cloud Postgres platform.
// It finds RAG and mission states requiring escalation and pushes them.
type HybridSyncDaemon struct {
	dbWrapper   *db.DB
	ticker      *time.Ticker
	quit        chan struct{}
	cloudAPIURL string
}

// NewHybridSyncDaemon creates a new local-to-cloud synchronization loop.
func NewHybridSyncDaemon(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *HybridSyncDaemon {
	return &HybridSyncDaemon{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
	}
}

// Start launches the background polling loop if in standalone SQLite mode.
func (d *HybridSyncDaemon) Start(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		slog.Info("sync: HybridSyncDaemon disabled (not running in standalone SQLite mode)")
		return
	}

	go func() {
		for {
			select {
			case <-d.ticker.C:
				d.ProcessSyncTick(ctx)
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

// Stop halts the daemon gracefully.
func (d *HybridSyncDaemon) Stop() {
	close(d.quit)
}

// ProcessSyncTick is exposed for synchronous tests and triggered by the ticker loop.
func (d *HybridSyncDaemon) ProcessSyncTick(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		return
	}

	d.syncMissions(ctx)
}

func (d *HybridSyncDaemon) syncMissions(ctx context.Context) {
	// Query local SQLite for items marked for cloud escalation (synced_to_cloud = false).
	// Always include LIMIT 100 to prevent memory explosion.
	rows, err := d.dbWrapper.Query(ctx, "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false LIMIT 100")
	if err != nil {
		slog.Error("sync: failed to query agent_missions", "error", err)
		return
	}
	defer rows.Close()

	var payloads []AutoDreamPayload
	var ids []string

	for rows.Next() {
		var id, status, payloadData string
		if err := rows.Scan(&id, &status, &payloadData); err != nil {
			slog.Error("sync: failed to scan agent_missions", "error", err)
			continue
		}

		payloads = append(payloads, AutoDreamPayload{
			Type:     "mission",
			ID:       id,
			Data:     payloadData,
			Metadata: status,
		})
		ids = append(ids, id)
	}

	if len(payloads) == 0 {
		return
	}

	if err := d.sendToCloud(ctx, payloads); err != nil {
		if telemetry.SyncFailedCount != nil {
			telemetry.SyncFailedCount.Add(ctx, int64(len(payloads)))
		}
		slog.Error("sync: failed to push missions to cloud", "error", err)
		return
	}

	// Update the local database to reflect successful sync
	for _, id := range ids {
		_, err := d.dbWrapper.Exec(ctx, "UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1", id)
		if err != nil {
			slog.Error("sync: failed to update synced_to_cloud status", "id", id, "error", err)
		}
	}

	if telemetry.SyncCompletedCount != nil {
		telemetry.SyncCompletedCount.Add(ctx, int64(len(payloads)))
	}
	slog.Info("sync: successfully synced missions to cloud", "count", len(payloads))
}

func (d *HybridSyncDaemon) sendToCloud(ctx context.Context, payloads []AutoDreamPayload) error {
	// Provide a graceful bypass for testing when no cloud URL is set
	if d.cloudAPIURL == "" {
		return nil
	}

	jsonData, err := json.Marshal(payloads)
	if err != nil {
		return fmt.Errorf("failed to marshal payload array: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, d.cloudAPIURL+"/api/sync/missions", bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("failed to create sync request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	// Apply authentication tokens, preferring specific API keys to environment variables
	if authHeader := os.Getenv("OHC_CLOUD_API_KEY"); authHeader != "" {
		req.Header.Set("Authorization", "Bearer "+authHeader)
	} else if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 15 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("sync POST failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("sync POST returned %d: %s", resp.StatusCode, string(body))
	}

	return nil
}
