package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric"
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
	Vector       []byte // Fetch as byte slice to be compatible with pgx bytea and sqlite blob
	SourcePlugin *string
	SyncStatus   SyncStatus
	LastSyncAt   time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DBRAGSyncService struct {
	db          db.Provider
	syncedTotal metric.Int64Counter
	errorsTotal metric.Int64Counter
}

func NewDBRAGSyncService(provider db.Provider, meter metric.Meter) (*DBRAGSyncService, error) {
	syncedTotal, err := meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		return nil, err
	}
	errorsTotal, err := meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		return nil, err
	}
	return &DBRAGSyncService{
		db:          provider,
		syncedTotal: syncedTotal,
		errorsTotal: errorsTotal,
	}, nil
}

func (s *DBRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus string
		var sourcePlugin *string
		var vector []byte
		if err := rows.Scan(&r.ID, &r.Context, &vector, &sourcePlugin, &syncStatus, &lastSyncAt); err != nil {
			s.errorsTotal.Add(ctx, 1)
			return nil, err
		}
		r.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		r.Vector = vector
		r.SourcePlugin = sourcePlugin
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *DBRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1", id)
		if err != nil {
			s.errorsTotal.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return err
	}

	s.syncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DBRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		_, err = tx.Exec(ctx, `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				source_plugin = excluded.source_plugin,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`, r.ID, r.Context, r.Vector, r.SourcePlugin)
		if err != nil {
			s.errorsTotal.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return err
	}

	s.syncedTotal.Add(ctx, int64(len(records)))
	return nil
}
