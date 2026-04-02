package sync

import (
	"bytes"
	"context"
	"encoding/json"
	"log"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// AutoDream is the payload format for syncing embeddings to the cloud.
type AutoDream struct {
	ContentHash string `json:"content_hash"`
	Embedding   []byte `json:"embedding"`
}

// AutoDreamSyncEngine is a background sidecar that synchronizes locally generated
// AutoDream insights (embeddings) to the cloud when in Standalone mode.
type AutoDreamSyncEngine struct {
	dbProvider db.Provider
	ticker     *time.Ticker
	httpClient *http.Client
	cloudURL   string
	done       chan struct{}
}

// NewAutoDreamSyncEngine initializes a new AutoDreamSyncEngine.
func NewAutoDreamSyncEngine(dbProvider db.Provider, cloudURL string) *AutoDreamSyncEngine {
	return &AutoDreamSyncEngine{
		dbProvider: dbProvider,
		cloudURL:   cloudURL,
		httpClient: &http.Client{Timeout: 10 * time.Second},
		done:       make(chan struct{}),
	}
}

// Start begins the synchronization loop.
func (e *AutoDreamSyncEngine) Start(interval time.Duration) {
	if !e.dbProvider.IsSQLite() {
		// Sync is only meaningful when running in Standalone Desktop mode.
		return
	}

	e.ticker = time.NewTicker(interval)
	go func() {
		for {
			select {
			case <-e.ticker.C:
				e.ProcessForecastTick()
			case <-e.done:
				return
			}
		}
	}()
}

// Stop halts the synchronization loop.
func (e *AutoDreamSyncEngine) Stop() {
	if e.ticker != nil {
		e.ticker.Stop()
	}
	close(e.done)
}

// ProcessForecastTick executes one round of synchronization synchronously.
func (e *AutoDreamSyncEngine) ProcessForecastTick() {
	ctx := context.Background()

	// Query unsynced embeddings
	rows, err := e.dbProvider.Query(ctx, "SELECT content_hash, embedding FROM embedding_cache WHERE synced_to_cloud = false LIMIT 100")
	if err != nil {
		log.Printf("Failed to query unsynced embeddings: %v", err)
		return
	}
	defer rows.Close()

	var payload []AutoDream
	var hashes []string

	for rows.Next() {
		var dream AutoDream
		if err := rows.Scan(&dream.ContentHash, &dream.Embedding); err != nil {
			log.Printf("Failed to scan row: %v", err)
			continue
		}
		payload = append(payload, dream)
		hashes = append(hashes, dream.ContentHash)
	}

	if len(payload) == 0 {
		return
	}

	body, err := json.Marshal(payload)
	if err != nil {
		log.Printf("Failed to marshal payload: %v", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, "POST", e.cloudURL+"/api/v1/sync/autodream", bytes.NewBuffer(body))
	if err != nil {
		log.Printf("Failed to create sync request: %v", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := e.httpClient.Do(req)
	if err != nil || resp.StatusCode != http.StatusOK {
		log.Printf("Failed to sync embeddings to cloud: %v", err)
		if telemetry.SyncFailedCount != nil {
			telemetry.SyncFailedCount.Add(ctx, int64(len(payload)))
		}
		if resp != nil {
			resp.Body.Close()
		}
		return
	}
	resp.Body.Close()

	if telemetry.SyncCompletedCount != nil {
		telemetry.SyncCompletedCount.Add(ctx, int64(len(payload)))
	}

	// Update synced status
	for _, hash := range hashes {
		_, err := e.dbProvider.Exec(ctx, "UPDATE embedding_cache SET synced_to_cloud = true WHERE content_hash = $1", hash)
		if err != nil {
			log.Printf("Failed to update sync status for %s: %v", hash, err)
		}
	}
}
