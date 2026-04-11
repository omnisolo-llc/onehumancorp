package hub

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"strings"
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
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter := otel.Meter("hub_rag_sync")

	var err error
	RecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records successfully synced to the cloud"))
	if err != nil {
		log.Printf("failed to initialize rag_records_synced_total metric: %v", err)
	}

	SyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors encountered during RAG sync"))
	if err != nil {
		log.Printf("failed to initialize rag_sync_errors_total metric: %v", err)
	}
}

type RAGSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{
		provider: provider,
	}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if !s.provider.IsSQLite() {
		// Standalone specific logic
		return nil, fmt.Errorf("FetchPendingSyncs is only supported in Standalone mode")
	}

	query := `
		SELECT id, context, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	if !s.provider.IsSQLite() {
		// Standalone specific logic
		return fmt.Errorf("MarkSynced is only supported in Standalone mode")
	}

	// Simple IN clause for SQLite
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = "?"
		args[i] = id
	}

	query := fmt.Sprintf(`
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE id IN (%s)
	`, strings.Join(placeholders, ","))

	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("failed to update sync status: %w", err)
	}

	// Metric update
	RecordsSyncedTotal.Add(ctx, int64(len(ids)))

	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	if s.provider.IsSQLite() {
		// Cloud specific logic
		return fmt.Errorf("ProcessIncomingSync is only supported in Cloud mode")
	}

	// This is a simplified upsert.
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO autodream_memories (id, context, sync_status, last_sync_at)
		VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT (id) DO UPDATE SET
			context = EXCLUDED.context,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
	`

	for _, r := range records {
		_, err := tx.Exec(ctx, query, r.ID, r.Context)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
