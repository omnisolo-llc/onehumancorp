package hub

import (
	"bytes"
	"context"
	"database/sql"
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

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID           string
	Context      string
	Vector       []float32
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type RAGSyncEngine struct {
	dbWrapper   *db.DB
	ticker      *time.Ticker
	quit        chan struct{}
	cloudAPIURL string
}

func NewRAGSyncEngine(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *RAGSyncEngine {
	return &RAGSyncEngine{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
	}
}

func (e *RAGSyncEngine) Start(ctx context.Context) {
	if !e.dbWrapper.IsSQLite() {
		// Only run sync engine in standalone/SQLite mode
		slog.Debug("sync: RAGSyncEngine disabled (not in standalone SQLite mode)")
		return
	}

	go func() {
		for {
			select {
			case <-e.ticker.C:
				e.ProcessSyncTick(ctx)
			case <-e.quit:
				e.ticker.Stop()
				return
			case <-ctx.Done():
				e.ticker.Stop()
				return
			}
		}
	}()
}

func (e *RAGSyncEngine) Stop() {
	close(e.quit)
}

func (e *RAGSyncEngine) ProcessSyncTick(ctx context.Context) {
	if !e.dbWrapper.IsSQLite() {
		return
	}

	records, err := e.FetchPendingSyncs(ctx, 50)
	if err != nil {
		slog.Error("sync: failed to fetch pending RAG syncs", "error", err)
		return
	}

	if len(records) == 0 {
		return
	}

	if err := e.sendToCloud(ctx, records); err != nil {
		if telemetry.RAGSyncErrorsTotal != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, int64(len(records)))
		}
		slog.Error("sync: failed to send RAG records to cloud", "error", err)
		return
	}

	var ids []string
	for _, rec := range records {
		ids = append(ids, rec.ID)
	}

	if err := e.MarkSynced(ctx, ids); err != nil {
		slog.Error("sync: failed to mark RAG records as synced", "error", err)
	} else {
		if telemetry.RAGRecordsSyncedTotal != nil {
			telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(records)))
		}
		slog.Debug("sync: successfully synced RAG records", "count", len(records))
	}
}

func (e *RAGSyncEngine) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = $1 LIMIT $2"
	rows, err := e.dbWrapper.Query(ctx, query, SyncStatusPending, limit)
	if err != nil {
		return nil, fmt.Errorf("query autodream_memories: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var vecStr sql.NullString
		var syncStatus string

		if err := rows.Scan(&rec.ID, &rec.Context, &vecStr, &syncStatus, &lastSyncAt); err != nil {
			slog.Error("sync: failed to scan autodream_memories", "error", err)
			continue
		}

		rec.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}

		// For now we just mock the vector as empty, in real env we might parse pgvector string to []float32
		rec.Vector = []float32{}

		records = append(records, rec)
	}

	return records, nil
}

func (e *RAGSyncEngine) MarkSynced(ctx context.Context, ids []string) error {
	for _, id := range ids {
		_, err := e.dbWrapper.Exec(ctx, "UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id = $3", SyncStatusSynced, time.Now(), id)
		if err != nil {
			return fmt.Errorf("update autodream_memories %s: %w", id, err)
		}
	}
	return nil
}

func (e *RAGSyncEngine) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Not implemented for engine, this would be on the cloud side API
	return nil
}

func (e *RAGSyncEngine) sendToCloud(ctx context.Context, payloads []RAGSyncRecord) error {
	// If cloudAPIURL is empty (e.g. testing), mock success
	if e.cloudAPIURL == "" {
		return nil
	}

	jsonData, err := json.Marshal(payloads)
	if err != nil {
		return fmt.Errorf("marshal payloads: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, e.cloudAPIURL, bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	// Set SPIFFE authentication token header if identity token is provided in environment variables.
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
