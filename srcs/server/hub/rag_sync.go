package hub

import (
	"context"
	"database/sql"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
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
	LastSyncAt *time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics(meter metric.Meter) {
	if meter == nil {
		return
	}
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
	if err != nil {
		otel.Handle(err)
	}
	ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
	if err != nil {
		otel.Handle(err)
	}
}

type dbRAGSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &dbRAGSyncService{provider: provider}
}

func (s *dbRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2`
	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var syncStatus string
		var vector []byte
		if err := rows.Scan(&rec.ID, &rec.Context, &vector, &syncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		rec.Vector = vector
		rec.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt.Valid {
			rec.LastSyncAt = &lastSyncAt.Time
		}
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *dbRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id = $3`
		if _, err := tx.Exec(ctx, query, string(SyncStatusSynced), now, id); err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
		if ragRecordsSyncedTotal != nil {
			ragRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return tx.Commit(ctx)
}

func (s *dbRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var lastSync interface{}
		if rec.LastSyncAt != nil {
			lastSync = *rec.LastSyncAt
		} else {
			lastSync = nil
		}

		query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				  VALUES ($1, $2, $3, $4, $5)
				  ON CONFLICT (memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = excluded.sync_status, last_sync_at = excluded.last_sync_at`
		_, err = tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, string(rec.SyncStatus), lastSync)

		if err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
		if ragRecordsSyncedTotal != nil {
			ragRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return tx.Commit(ctx)
}
