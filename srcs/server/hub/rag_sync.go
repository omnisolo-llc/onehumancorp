package hub

import (
	"context"
	"fmt"
	"log/slog"
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

type RAGSyncServiceImpl struct {
	dbWrapper db.Provider
}

func NewRAGSyncService(dbWrapper db.Provider) RAGSyncService {
	return &RAGSyncServiceImpl{
		dbWrapper: dbWrapper,
	}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if !s.dbWrapper.IsSQLite() {
		return nil, fmt.Errorf("FetchPendingSyncs is only supported in SQLite mode")
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	rows, err := tx.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var vector []byte
		if err := rows.Scan(&r.ID, &r.Context, &vector, &r.SyncStatus, &lastSyncAt); err != nil {
			slog.Error("failed to scan pending sync record", "error", err)
			continue
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		r.Vector = vector
		records = append(records, r)
	}
	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if !s.dbWrapper.IsSQLite() {
		return fmt.Errorf("MarkSynced is only supported in SQLite mode")
	}
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC()
	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2"
		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGSyncSuccess(ctx, int64(len(ids)))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if s.dbWrapper.IsSQLite() {
		return fmt.Errorf("ProcessIncomingSync is only supported in PostgreSQL mode")
	}
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC()
	for _, r := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = EXCLUDED.last_sync_at
		`
		if _, err := tx.Exec(ctx, query, r.ID, r.Context, r.Vector, now); err != nil {
			telemetry.RecordRAGSyncError(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordRAGSyncSuccess(ctx, int64(len(records)))
	return nil
}
