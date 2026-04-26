package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"time"
	"github.com/onehumancorp/mono/src/server/db"
)

type RagSyncDaemon struct {
	dbWrapper   *db.DB
	ticker      *time.Ticker
	quit        chan struct{}
	cloudAPIURL string
}

func NewRagSyncDaemon(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *RagSyncDaemon {
	if cloudAPIURL == "" {
		cloudAPIURL = os.Getenv("OHC_CORE_URL")
	}
	if cloudAPIURL == "" {
		cloudAPIURL = "http://localhost:8080"
	}
	return &RagSyncDaemon{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
	}
}

func (d *RagSyncDaemon) Start(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		slog.Debug("rag_sync_daemon: Disabled (not in standalone mode)")
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
			case <-d.ticker.C:
				d.ProcessSync(ctx)
			}
		}
	}()
}

func (d *RagSyncDaemon) Stop() {
	close(d.quit)
}

type RagSyncRecord struct {
	ID      string `json:"id"`
	Context string `json:"context"`
	Status  string `json:"status"`
}

func (d *RagSyncDaemon) ProcessSync(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		return
	}
	tx, err := d.dbWrapper.Begin(ctx)
	if err != nil {
		return
	}
	defer tx.Rollback(ctx)

	query := "SELECT memory_id, context FROM swarm_memory_embeddings LIMIT 500"
	rows, err := tx.Query(ctx, query)
	if err != nil {
		return
	}
	defer rows.Close()

	var records []RagSyncRecord
	for rows.Next() {
		var id, contextData string
		if err := rows.Scan(&id, &contextData); err == nil {
			records = append(records, RagSyncRecord{
				ID:      id,
				Context: contextData,
				Status:  "pending",
			})
		}
	}

	if len(records) == 0 {
		return
	}

	payload := map[string]interface{}{"records": records}
	jsonData, err := json.Marshal(payload)
	if err != nil {
		return
	}

	syncEndpoint := fmt.Sprintf("%s/api/mcp/rag/sync", d.cloudAPIURL)
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(jsonData))
	if err != nil {
		return
	}
	req.Header.Set("Content-Type", "application/json")
	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 200 && resp.StatusCode < 300 {
		// Treat as synced, we could delete or mark them if needed
		slog.Debug("rag_sync_daemon: successfully synced records")
	}
}
