package hub

import (
	"context"
	"fmt"
	"strings"
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

type ragSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		provider: provider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.provider.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2", SyncStatusPending, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return nil, err
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create placeholder string "$1, $2, $3..."
	placeholders := make([]string, len(ids))
	args := make([]any, len(ids)+2)
	args[0] = SyncStatusSynced
	args[1] = time.Now()

	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+3)
		args[i+2] = id
	}

	query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id IN (%s)", strings.Join(placeholders, ", "))

	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}

	telemetry.RecordRAGSyncSuccess(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (memory_id) DO UPDATE
		SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := tx.Exec(ctx, query, r.ID, r.Context, r.Vector, SyncStatusSynced, time.Now())
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	telemetry.RecordRAGSyncSuccess(ctx, int64(len(records)))
	return nil
}
