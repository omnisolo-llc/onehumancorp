package hub

import (
	"context"
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
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &RAGSyncServiceImpl{db: db}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, embedding, sync_status, last_sync_timestamp FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	if s.db.IsSQLite() {
		query = "SELECT id, content, embedding, sync_status, last_sync_timestamp FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?1"
	}
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSync *time.Time
		var statusStr *string
		var embeddingStr *string

		if err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr, &statusStr, &lastSync); err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if statusStr != nil {
			rec.SyncStatus = SyncStatus(*statusStr)
		} else {
			rec.SyncStatus = SyncStatusPending
		}

		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}

		if embeddingStr != nil {
			// Hybrid compat: in SQLite we might store vector as a JSON string, in pgvector we might receive text when cast
			_ = json.Unmarshal([]byte(*embeddingStr), &rec.Vector)
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Update sync_status and last_sync_timestamp
	now := time.Now()

	tx, err := s.db.Begin(ctx)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_timestamp = $1 WHERE id = $2"
		if s.db.IsSQLite() {
			query = "UPDATE autodream_memories SET sync_status = 'synced', last_sync_timestamp = ?1 WHERE id = ?2"
		}

		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGSyncSuccess(ctx, len(ids))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		embeddingStr, err := json.Marshal(rec.Vector)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to marshal vector: %w", err)
		}

		if s.db.IsSQLite() {
			query := `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
				VALUES (?1, ?2, ?3, 'synced', ?4)
				ON CONFLICT(id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = 'synced',
					last_sync_timestamp = excluded.last_sync_timestamp
			`
			_, err = tx.Exec(ctx, query, rec.ID, rec.Context, string(embeddingStr), rec.LastSyncAt)
		} else {
			// Postgres pgvector
			query := `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
				VALUES ($1, $2, CASE WHEN $3::text = 'null' OR $3::text IS NULL THEN NULL ELSE $3::text::vector END, 'synced', $4)
				ON CONFLICT(id) DO UPDATE SET
					content = EXCLUDED.content,
					embedding = EXCLUDED.embedding,
					sync_status = 'synced',
					last_sync_timestamp = EXCLUDED.last_sync_timestamp
			`
			_, err = tx.Exec(ctx, query, rec.ID, rec.Context, string(embeddingStr), rec.LastSyncAt)
		}

		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to process incoming record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGSyncSuccess(ctx, len(records))
	return nil
}
