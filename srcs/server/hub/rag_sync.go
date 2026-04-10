package hub

import (
	"context"
	"database/sql"
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
	Vector     []byte
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

type DefaultRAGSyncService struct {
	dbProvider *db.DB
}

func NewDefaultRAGSyncService(dbProvider *db.DB) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		dbProvider: dbProvider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	if s.dbProvider.IsSQLite() {
		query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT ?"
	}

	rows, err := tx.Query(ctx, query, limit)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return nil, fmt.Errorf("failed to execute query: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSyncAt); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()

	// SQLite limitation: cannot use = ANY($1) array operators easily, doing sequential updates or IN clause
	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2"
		if s.dbProvider.IsSQLite() {
			query = "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = ? WHERE memory_id = ?"
		}

		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRagRecordSynced(ctx, len(ids))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()

	for _, rec := range records {
		query := "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', $4) ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = EXCLUDED.last_sync_at"
		if s.dbProvider.IsSQLite() {
			query = "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES (?, ?, ?, 'synced', ?) ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = EXCLUDED.last_sync_at"
		}

		if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, now); err != nil {
			telemetry.RecordRagSyncError(ctx)
			return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRagSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRagRecordSynced(ctx, len(records))
	return nil
}
