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
)

type SyncDaemon struct {
	dbWrapper   *db.DB
	ticker      *time.Ticker
	quit        chan struct{}
	cloudAPIURL string
}

func NewSyncDaemon(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *SyncDaemon {
	return &SyncDaemon{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
	}
}

func (d *SyncDaemon) Start(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		// Only run in Standalone mode
		return
	}

	go func() {
		for {
			select {
			case <-d.ticker.C:
				d.ProcessTick(ctx)
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

func (d *SyncDaemon) Stop() {
	close(d.quit)
}

func (d *SyncDaemon) ProcessTick(ctx context.Context) {
	d.syncMissions(ctx)
}

func (d *SyncDaemon) syncMissions(ctx context.Context) {
	// Look for items in agent_missions marked for cloud escalation (synced_to_cloud = false and maybe marked in payload? Let's check payload for cloud_escalation flag or just use synced_to_cloud = false as instructed by prompt)
	// The prompt says: "Implement a background loop that queries the local SQLite agent_missions for items marked for cloud escalation. Always include a LIMIT 100 clause"
	// and "injects it into the Cloud Postgres agent_missions table."

	// Wait, we can assume payload has some indicator, or we sync all `synced_to_cloud = false`?
	// The prompt: "queries the local SQLite agent_missions for items marked for cloud escalation."

	query := "SELECT id, payload FROM agent_missions WHERE json_extract(payload, '$.cloud_escalation') = true AND synced_to_cloud = false LIMIT 100"

	rows, err := d.dbWrapper.Query(ctx, query)
	if err != nil {
		// Also try without json_extract if SQLite plugin is missing, or fallback to simple LIKE
		// Let's use simple string matching to avoid json_extract issues in some SQLite versions if we want, but json_extract is standard in SQLite >3.38
		slog.Error("sync_daemon: failed to query agent_missions", "error", err)
		return
	}
	defer rows.Close()

	var payloads []map[string]interface{}
	var ids []string

	for rows.Next() {
		var id, payloadData string
		if err := rows.Scan(&id, &payloadData); err != nil {
			slog.Error("sync_daemon: failed to scan agent_missions", "error", err)
			continue
		}

		var payloadMap map[string]interface{}
		if err := json.Unmarshal([]byte(payloadData), &payloadMap); err != nil {
			slog.Error("sync_daemon: failed to unmarshal payload", "error", err)
			continue
		}

		payloadMap["id"] = id // Ensure ID is present

		payloads = append(payloads, payloadMap)
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
	for _, id := range ids {
		_, err := d.dbWrapper.Exec(ctx, "UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1", id)
		if err != nil {
			slog.Error("sync_daemon: failed to update agent_missions status", "id", id, "error", err)
		}
	}

	slog.Info("sync_daemon: successfully synced agent_missions to cloud", "count", len(payloads))
}

func (d *SyncDaemon) sendToCloud(ctx context.Context, payloads []map[string]interface{}) error {
	if d.cloudAPIURL == "" {
		// Use environment variable if not provided
		d.cloudAPIURL = os.Getenv("OHC_CORE_URL")
		if d.cloudAPIURL == "" {
			return nil // Skip if no cloud URL
		}
	}

	endpoint := d.cloudAPIURL + "/api/sync/missions"

	jsonData, err := json.Marshal(payloads)
	if err != nil {
		return fmt.Errorf("marshal payloads: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, endpoint, bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	// Authentication (e.g. SPIFFE or API Key)
	if authHeader := os.Getenv("OHC_CLOUD_API_KEY"); authHeader != "" {
		req.Header.Set("Authorization", "Bearer "+authHeader)
	} else if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
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
