package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SyncDaemon synchronizes local SQLite agent_missions to the Cloud Postgres agent_missions.
type SyncDaemon struct {
	dbProvider  db.Provider
	cloudURL    string
	httpClient  *http.Client
}

// NewSyncDaemon creates a new daemon for Offline-to-Cloud Sync.
func NewSyncDaemon(dbProvider db.Provider, cloudURL string) *SyncDaemon {
	return &SyncDaemon{
		dbProvider: dbProvider,
		cloudURL:   cloudURL,
		httpClient: &http.Client{Timeout: 10 * time.Second},
	}
}

// StartDaemon starts the background loop.
func (d *SyncDaemon) StartDaemon(ctx context.Context, tickDuration time.Duration) {
	ticker := time.NewTicker(tickDuration)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			d.ProcessSyncTick(ctx)
		}
	}
}

type syncMissionPayload struct {
	ID          int64  `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`
	Status      string `json:"status"`
}

// ProcessSyncTick performs one iteration of the synchronization logic.
func (d *SyncDaemon) ProcessSyncTick(ctx context.Context) {
	// Query local SQLite agent_missions for items requiring cloud escalation.
	// We'll look for items that are not explicitly marked 'DONE'.
	// Using LIMIT 100 to prevent unbounded memory explosion.
	query := `
		SELECT id, title, description, status
		FROM agent_missions
		WHERE status != 'DONE' AND status != 'SYNCED'
		LIMIT 100
	`
	rows, err := d.dbProvider.Query(ctx, query)
	if err != nil {
		slog.Error("SyncDaemon: Failed to query agent_missions", "error", err)
		return
	}
	defer rows.Close()

	var payloads []syncMissionPayload
	for rows.Next() {
		var p syncMissionPayload
		if err := rows.Scan(&p.ID, &p.Title, &p.Description, &p.Status); err != nil {
			slog.Error("SyncDaemon: Failed to scan agent_mission", "error", err)
			continue
		}
		payloads = append(payloads, p)
	}

	if err := rows.Err(); err != nil {
		slog.Error("SyncDaemon: Rows error", "error", err)
		return
	}

	for _, p := range payloads {
		// Securely format the local payload and make an HTTP POST to the cloud API.
		bodyBytes, err := json.Marshal(p)
		if err != nil {
			slog.Error("SyncDaemon: Failed to marshal payload", "error", err)
			continue
		}

		req, err := http.NewRequestWithContext(ctx, http.MethodPost, d.cloudURL+"/api/sync/missions", bytes.NewReader(bodyBytes))
		if err != nil {
			slog.Error("SyncDaemon: Failed to create request", "error", err)
			continue
		}
		req.Header.Set("Content-Type", "application/json")
		// Ideally we would add authentication headers here if available

		resp, err := d.httpClient.Do(req)
		if err != nil {
			slog.Error("SyncDaemon: Failed to send sync request", "error", err)
			continue
		}

		bodyData, _ := io.ReadAll(resp.Body)
		resp.Body.Close()

		if resp.StatusCode >= 200 && resp.StatusCode < 300 {
			// Mark as synced locally
			updateQuery := `UPDATE agent_missions SET status = 'SYNCED' WHERE id = $1`
			_, err = d.dbProvider.Exec(ctx, updateQuery, p.ID)
			if err != nil {
				slog.Error("SyncDaemon: Failed to update local mission status to SYNCED", "error", err)
			} else {
				slog.Info("SyncDaemon: Successfully synced mission to cloud", "id", p.ID)
			}
		} else {
			slog.Error("SyncDaemon: Cloud sync failed", "status", resp.StatusCode, "response", string(bodyData))
		}
	}
}
