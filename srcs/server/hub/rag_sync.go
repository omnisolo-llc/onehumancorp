package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
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
	ID         string
	Context    string
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type RAGSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{
		provider: provider,
	}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	if s.provider.IsSQLite() {
		query = `SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?`
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingStr sql.NullString
		var status sql.NullString
		var lastSync sql.NullTime

		if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &status, &lastSync); err != nil {
			telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if embeddingStr.Valid && embeddingStr.String != "" {
			// Basic vector parsing, actual Vector struct might be more complex
			var vector []float32
			if err := json.Unmarshal([]byte(embeddingStr.String), &vector); err == nil {
				r.Vector = vector
			}
		}

		if status.Valid {
			r.SyncStatus = SyncStatus(status.String)
		} else {
			r.SyncStatus = SyncStatusPending
		}

		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}

		records = append(records, r)
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// For simplicity in sqlite vs postgres, we execute queries iteratively or use IN clause
	// A simpler robust way across both is a transaction
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`
	if s.provider.IsSQLite() {
		query = `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ?`
	}

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	telemetry.RecordRAGRecordsSyncedTotal(ctx, int64(len(ids)))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	// Use an upsert strategy
	var query string
	if s.provider.IsSQLite() {
		query = `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
                 VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
                 ON CONFLICT(id) DO UPDATE SET content=excluded.content, embedding=excluded.embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
	} else {
		query = `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
                 VALUES ($1, $2, $3::vector, 'synced', CURRENT_TIMESTAMP)
                 ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
	}

	for _, r := range records {
		var embStr interface{} = nil
		if len(r.Vector) > 0 {
			b, _ := json.Marshal(r.Vector)
			embStr = string(b)
		}

		if s.provider.IsSQLite() && embStr == nil {
			embStr = "[0.0]"
		} else if embStr == nil {
			embStr = "[0.0]"
		}

		_, err := tx.Exec(ctx, query, r.ID, r.Context, embStr)
		if err != nil {
			telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRAGSyncErrorsTotal(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	return nil
}
