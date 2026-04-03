package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type SyncDaemon struct {
	dbWrapper   db.Provider
	remoteURL   string
	ticker      *time.Ticker
	quit        chan struct{}
	httpClient  *http.Client
}

func NewSyncDaemon(dbWrapper db.Provider) *SyncDaemon {
	remoteURL := os.Getenv("OHC_CORE_URL")
	if remoteURL == "" {
		remoteURL = "http://localhost:8080" // Fallback
	}
	return &SyncDaemon{
		dbWrapper:  dbWrapper,
		remoteURL:  remoteURL,
		ticker:     time.NewTicker(30 * time.Second),
		quit:       make(chan struct{}),
		httpClient: &http.Client{Timeout: 10 * time.Second},
	}
}

func (d *SyncDaemon) Start() {
	go func() {
		for {
			select {
			case <-d.ticker.C:
				d.ProcessSyncTick()
			case <-d.quit:
				return
			}
		}
	}()
}

func (d *SyncDaemon) Stop() {
	d.ticker.Stop()
	close(d.quit)
}

func (d *SyncDaemon) ProcessSyncTick() {
	ctx := context.Background()

	// "Always include a LIMIT 100 clause in your background sync loop SELECT query to prevent memory explosion."
	// Query local SQLite for items marked for cloud escalation.
	// For offline-to-cloud sync, we find items that haven't been synced to cloud yet.
	// We'll check synced_to_cloud = false and perhaps limit to missions that have content.
	// The problem statement says: "When a local MCP agent detects a task requiring scalable compute, it orchestrates a synchronization. It sanitizes and packages the minimal required local context payload (from SQLite agent_missions / RAG DB) and injects it into the Cloud Postgres agent_missions table."

	// Because of SQLite support with JSON functions, we can extract fields, but a simpler generic query is just:
	rows, err := d.dbWrapper.Query(ctx, "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false LIMIT 100")
	if err != nil {
		slog.Error("sync_daemon: failed to query agent_missions", "error", err)
		return
	}
	defer rows.Close()

	var payloads []map[string]interface{}
	var ids []string

	for rows.Next() {
		var id, status, payloadStr string
		if err := rows.Scan(&id, &status, &payloadStr); err != nil {
			slog.Error("sync_daemon: failed to scan agent_missions", "error", err)
			continue
		}

		var payloadObj interface{}
		if err := json.Unmarshal([]byte(payloadStr), &payloadObj); err != nil {
			payloadObj = map[string]interface{}{"raw": payloadStr}
		}

		missionData := map[string]interface{}{
			"id":      id,
			"status":  status,
			"payload": payloadObj,
		}

		payloads = append(payloads, missionData)
		ids = append(ids, id)
	}

	if len(payloads) == 0 {
		return
	}

	reqBody, err := json.Marshal(map[string]interface{}{
		"missions": payloads,
	})
	if err != nil {
		slog.Error("sync_daemon: failed to marshal payload", "error", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, "POST", d.remoteURL+"/api/sync/missions", bytes.NewBuffer(reqBody))
	if err != nil {
		slog.Error("sync_daemon: failed to create request", "error", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")

	// Authentication (e.g. SPIFFE/SPIRE via interceptors or simple Bearer for now)
	token := os.Getenv("OHC_SYNC_TOKEN")
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}

	resp, err := d.httpClient.Do(req)
	if err != nil {
		slog.Error("sync_daemon: failed to send sync request", "error", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 200 && resp.StatusCode < 300 {
		// Mark as synced locally
		for _, id := range ids {
			_, err := d.dbWrapper.Exec(ctx, "UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1", id)
			if err != nil {
				slog.Error("sync_daemon: failed to update synced_to_cloud", "id", id, "error", err)
			}
		}
		slog.Info("sync_daemon: successfully synced missions to cloud", "count", len(ids))
	} else {
		slog.Error("sync_daemon: received non-200 status from cloud", "status", resp.StatusCode)
	}
}
