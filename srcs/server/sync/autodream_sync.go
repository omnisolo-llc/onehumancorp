package sync

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// AutoDream is the payload struct for the API contract
type AutoDream struct {
	ContentHash string `json:"content_hash"`
	Embedding   string `json:"embedding"`
	CreatedAt   string `json:"created_at"`
}

type AutoDreamSyncEngine struct {
	dbProvider db.Provider
	ticker     *time.Ticker
	quit       chan struct{}
	client     *http.Client
}

func NewAutoDreamSyncEngine(provider db.Provider) *AutoDreamSyncEngine {
	return &AutoDreamSyncEngine{
		dbProvider: provider,
		quit:       make(chan struct{}),
		client:     &http.Client{Timeout: 10 * time.Second},
	}
}

// Start begins the synchronization loop.
func (e *AutoDreamSyncEngine) Start(interval time.Duration) {
	e.ticker = time.NewTicker(interval)
	go func() {
		for {
			select {
			case <-e.ticker.C:
				e.ProcessForecastTick(context.Background())
			case <-e.quit:
				e.ticker.Stop()
				return
			}
		}
	}()
}

// Stop halts the synchronization loop.
func (e *AutoDreamSyncEngine) Stop() {
	close(e.quit)
}

// ProcessForecastTick performs the actual synchronization work.
// Extracted to a synchronous method to ensure reliable test coverage.
func (e *AutoDreamSyncEngine) ProcessForecastTick(ctx context.Context) {
	// Only run this if we are running locally with SQLite
	if !e.dbProvider.IsSQLite() {
		return
	}

	cloudURL := os.Getenv("OHC_CLOUD_API_URL")
	if cloudURL == "" {
		// Default to local/test mock endpoint if not set, or we can just return.
		cloudURL = "http://localhost:8080"
	}

	// Fetch unsynced items
	rows, err := e.dbProvider.Query(ctx, `
		SELECT content_hash, embedding, created_at
		FROM embedding_cache
		WHERE synced_to_cloud = false
	`)
	if err != nil {
		slog.Error("autodream_sync: failed to query unsynced embeddings", "error", err)
		return
	}
	defer rows.Close()

	var unsynced []AutoDream
	for rows.Next() {
		var d AutoDream
		if err := rows.Scan(&d.ContentHash, &d.Embedding, &d.CreatedAt); err != nil {
			slog.Error("autodream_sync: failed to scan row", "error", err)
			continue
		}
		unsynced = append(unsynced, d)
	}

	if err := rows.Err(); err != nil {
		slog.Error("autodream_sync: rows iteration error", "error", err)
		return
	}

	if len(unsynced) == 0 {
		return
	}

	// Send to cloud
	payload, err := json.Marshal(unsynced)
	if err != nil {
		slog.Error("autodream_sync: failed to marshal payload", "error", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, "POST", fmt.Sprintf("%s/api/v1/sync/autodream", cloudURL), bytes.NewBuffer(payload))
	if err != nil {
		slog.Error("autodream_sync: failed to create request", "error", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")

	// We might need to handle auth, but for now we assume SPIFFE/SPIRE or local network
	resp, err := e.client.Do(req)
	if err != nil {
		telemetry.RecordSyncFailed(ctx, "sqlite", err.Error())
		slog.Error("autodream_sync: failed to send sync request", "error", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		telemetry.RecordSyncFailed(ctx, "sqlite", fmt.Sprintf("http_%d", resp.StatusCode))
		slog.Error("autodream_sync: cloud sync returned non-200", "status", resp.StatusCode)
		return
	}

	// Mark as synced
	for _, item := range unsynced {
		_, err := e.dbProvider.Exec(ctx, `
			UPDATE embedding_cache
			SET synced_to_cloud = true
			WHERE content_hash = $1
		`, item.ContentHash)
		if err != nil {
			telemetry.RecordSyncFailed(ctx, "sqlite", err.Error())
			slog.Error("autodream_sync: failed to mark item as synced", "hash", item.ContentHash, "error", err)
		} else {
			telemetry.RecordSyncCompleted(ctx, "sqlite")
		}
	}
}
