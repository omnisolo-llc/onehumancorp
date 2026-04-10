package hub

import (
	"context"
	"time"
	"fmt"

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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type RAGSyncServiceImpl struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{db: db}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit <= 0 {
		limit = 100
	}

	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus *string

		if err := rows.Scan(&r.ID, &r.Context, &syncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		if syncStatus != nil {
			r.SyncStatus = SyncStatus(*syncStatus)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error iterating sync records: %w", err)
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_at = $1
			WHERE id = $2
		`
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			return fmt.Errorf("failed to mark record as synced (id: %s): %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	RecordRAGSyncSuccess(ctx, int64(len(ids)))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, r := range records {
		// Basic upsert depending on Postgres or SQLite
		if s.db.IsSQLite() {
			query := `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
				VALUES ($1, $2, 'synced', $3)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					sync_status = 'synced',
					last_sync_at = excluded.last_sync_at
			`
			_, err = tx.Exec(ctx, query, r.ID, r.Context, now)
		} else {
			query := `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
				VALUES ($1, $2, 'synced', $3)
				ON CONFLICT (id) DO UPDATE SET
					content = EXCLUDED.content,
					sync_status = 'synced',
					last_sync_at = EXCLUDED.last_sync_at
			`
			_, err = tx.Exec(ctx, query, r.ID, r.Context, now)
		}

		if err != nil {
			RecordRAGSyncError(ctx, 1)
			return fmt.Errorf("failed to upsert incoming sync record (id: %s): %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	RecordRAGSyncSuccess(ctx, int64(len(records)))
	return nil
}

var (
	meter                 = otel.Meter("ohc.rag_sync")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"ohc.rag_sync.records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"ohc.rag_sync.errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

func RecordRAGSyncSuccess(ctx context.Context, count int64) {
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, count)
	}
}

func RecordRAGSyncError(ctx context.Context, count int64) {
	if ragSyncErrorsTotal != nil {
		ragSyncErrorsTotal.Add(ctx, count)
	}
}
