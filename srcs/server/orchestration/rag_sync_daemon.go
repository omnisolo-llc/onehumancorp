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
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
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

	query := "SELECT memory_id, context FROM swarm_memory_embeddings WHERE sync_status = 'pending' OR sync_status IS NULL LIMIT 500"
	rows, err := tx.Query(ctx, query)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, err.Error())
		return
	}
	defer rows.Close()

	var records []RagSyncRecord
	var ids []string
	for rows.Next() {
		var id, contextData string
		if err := rows.Scan(&id, &contextData); err == nil {
			records = append(records, RagSyncRecord{
				ID:      id,
				Context: contextData,
				Status:  "pending",
			})
			ids = append(ids, id)
		}
	}

	if len(records) == 0 {
		return
	}

	payload := map[string]interface{}{"records": records}
	jsonData, err := json.Marshal(payload)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, err.Error())
		return
	}

	syncEndpoint := fmt.Sprintf("%s/api/mcp/rag/sync", d.cloudAPIURL)
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(jsonData))
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, err.Error())
		return
	}
	req.Header.Set("Content-Type", "application/json")
	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, err.Error())
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 200 && resp.StatusCode < 300 {
		if len(ids) > 0 {
			placeholders := ""
			args := make([]interface{}, len(ids))
			for i, id := range ids {
				if i > 0 {
					placeholders += ","
				}
				placeholders += "?"
				args[i] = id
			}
			updateQuery := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (%s)", placeholders)
			_, err := tx.Exec(ctx, updateQuery, args...)
			if err != nil {
				slog.Error("rag_sync_daemon: failed to update sync_status", "error", err)
				telemetry.RecordRAGSyncError(ctx, err.Error())
				return
			}
		}
		if err := tx.Commit(ctx); err != nil {
			slog.Error("rag_sync_daemon: failed to commit transaction", "error", err)
			telemetry.RecordRAGSyncError(ctx, err.Error())
			return
		}
		telemetry.RecordRAGRecordsSynced(ctx, int64(len(records)))
		slog.Debug("rag_sync_daemon: successfully synced records")
	} else {
		telemetry.RecordRAGSyncError(ctx, fmt.Sprintf("unexpected status code: %d", resp.StatusCode))
	}
}