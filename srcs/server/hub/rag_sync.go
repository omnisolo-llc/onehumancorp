package hub

import (
	"context"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"

	"github.com/onehumancorp/mono/srcs/server/db"
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

type defaultRAGSyncService struct {
	db db.Provider
}

// NewRAGSyncService creates a new RAGSyncService
func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &defaultRAGSyncService{
		db: db,
	}
}

func (s *defaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *defaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// This is a simple implementation assuming ids is small enough for multiple queries
	// or using ANY($1) for postgres. For SQLite compatibility, loop.
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2`

	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			if SyncErrorsTotal != nil {
				SyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if RecordsSyncedTotal != nil {
		RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *defaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`
	// Note: Proper upsert depends on database dialect.
	// SQLite allows ON CONFLICT, Postgres allows ON CONFLICT. This is a simplified approach.

	for _, rec := range records {
		var lastSync *time.Time
		if !rec.LastSyncAt.IsZero() {
			lastSync = &rec.LastSyncAt
		}

		if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, "synced", lastSync); err != nil {
			if SyncErrorsTotal != nil {
				SyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	return tx.Commit(ctx)
}

var (
	meter metric.Meter

	// Metrics
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

	var err error
	RecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	SyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}
