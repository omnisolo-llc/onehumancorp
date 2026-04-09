package hybrid_sync

import (
	"context"
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
	Vector     []byte // Converted from float32 array per Memory rules (BYTEA mapped to []byte)
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

var (
	meter                = otel.Meter("hybrid_mcp_rag_protocol")
	RecordsSyncedTotal   metric.Int64Counter
	SyncErrorsTotal      metric.Int64Counter
)

func init() {
	var err error
	RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

// sqlRAGSyncService is a SQL-backed implementation of RAGSyncService
type sqlRAGSyncService struct {
	db *db.DB
}

func NewSQLRAGSyncService(db *db.DB) RAGSyncService {
	return &sqlRAGSyncService{db: db}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = $1 OR sync_status IS NULL
		LIMIT $2
	`
	rows, err := s.db.Query(ctx, query, SyncStatusPending, limit)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var syncStatus *string
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &syncStatus, &lastSyncAt); err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return nil, err
		}
		if syncStatus != nil {
			rec.SyncStatus = SyncStatus(*syncStatus)
		} else {
			rec.SyncStatus = SyncStatusPending
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}

	return records, nil
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	now := time.Now()

	tx, err := s.db.Begin(ctx)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id = $3`
		_, err := tx.Exec(ctx, query, SyncStatusSynced, now, id)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	now := time.Now()

	tx, err := s.db.Begin(ctx)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, SyncStatusSynced, now)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}

	return nil
}
