package hub

import (
	"context"
	"log/slog"
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
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

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		slog.Error("Failed to initialize rag_records_synced_total metric", "error", err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		slog.Error("Failed to initialize rag_sync_errors_total metric", "error", err)
	}
}

// syncServiceImpl implements RAGSyncService.
type syncServiceImpl struct {
	db db.Provider
}

// NewRAGSyncService creates a new RAGSyncService.
func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &syncServiceImpl{db: db}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing.
// Uses FOR UPDATE SKIP LOCKED to ensure multiple concurrent processes do not pick up the same records.
func (s *syncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// The db.Provider's convertBindVars function will automatically strip FOR UPDATE SKIP LOCKED
	// when running in SQLite, but preserve it for PostgreSQL.
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		ORDER BY created_at ASC
		LIMIT $1
		FOR UPDATE SKIP LOCKED
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

// MarkSynced updates the local DB after a successful sync to the cloud.
func (s *syncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB.
func (s *syncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	// In OHC Hybrid DB Provider, SQLite supports standard INSERT ... ON CONFLICT
	// Upsert query directly using ON CONFLICT avoids transaction rollbacks caused by
	// concurrent select/insert races.
	upsertQuery := `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT(id) DO UPDATE SET
			content = excluded.content,
			sync_status = excluded.sync_status,
			last_sync_at = excluded.last_sync_at
	`

	for _, rec := range records {
		_, err := tx.Exec(ctx, upsertQuery, rec.ID, rec.Context)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
