package hub

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                 = otel.Meter("hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to the cloud"),
	)
	if err != nil {
		fmt.Printf("failed to initialize rag_records_synced_total metric: %v\n", err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors during RAG sync"),
	)
	if err != nil {
		fmt.Printf("failed to initialize rag_sync_errors_total metric: %v\n", err)
	}
}

// RecordRAGSyncSuccess increments the rag_records_synced_total counter.
func RecordRAGSyncSuccess(ctx context.Context, count int64) {
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, count)
	}
}

// RecordRAGSyncError increments the rag_sync_errors_total counter.
func RecordRAGSyncError(ctx context.Context) {
	if ragSyncErrorsTotal != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
	}
}

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

type DefaultRAGSyncService struct {
	dbProvider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		dbProvider: provider,
	}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		ORDER BY created_at ASC
		LIMIT $1
	`
	if s.dbProvider.IsSQLite() {
		query = `
			SELECT id, content, sync_status, last_sync_at
			FROM autodream_memories
			WHERE sync_status = 'pending'
			ORDER BY created_at ASC
			LIMIT ?
		`
	}

	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		RecordRAGSyncError(ctx)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus *string
		if err := rows.Scan(&rec.ID, &rec.Context, &syncStatus, &lastSyncAt); err != nil {
			RecordRAGSyncError(ctx)
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
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
		RecordRAGSyncError(ctx)
		return nil, fmt.Errorf("error iterating over pending syncs: %w", err)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`
	if s.dbProvider.IsSQLite() {
		query = `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE id = ?
		`
	}

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	RecordRAGSyncSuccess(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Since vector embeddings are specific and schema may differ, we focus on storing context for now or just upserting.
	// Last write wins on conflict based on ID.
	query := `
		INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
		VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`
	if s.dbProvider.IsSQLite() {
		query = `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES (?, ?, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
	}

	for _, rec := range records {
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context)
		if err != nil {
			RecordRAGSyncError(ctx)
			return fmt.Errorf("failed to upsert incoming sync record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RecordRAGSyncError(ctx)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	RecordRAGSyncSuccess(ctx, int64(len(records)))
	return nil
}
