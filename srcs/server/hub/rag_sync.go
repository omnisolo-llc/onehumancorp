package hub

import (
	"context"
	"fmt"
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
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type RAGSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &RAGSyncServiceImpl{provider: provider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	if s.provider.IsSQLite() {
		query = `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?`
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		SyncErrorsCounter.Add(ctx, 1)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus string
		if err := rows.Scan(&r.ID, &r.Context, &syncStatus, &lastSyncAt); err != nil {
			SyncErrorsCounter.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		r.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		SyncErrorsCounter.Add(ctx, 1)
		return nil, fmt.Errorf("error iterating rows: %w", err)
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		if s.provider.IsSQLite() {
			placeholders[i] = "?"
		} else {
			placeholders[i] = fmt.Sprintf("$%d", i+1)
		}
		args[i] = id
	}

	query := fmt.Sprintf(`UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ","))

	if _, err := s.provider.Exec(ctx, query, args...); err != nil {
		SyncErrorsCounter.Add(ctx, 1)
		return fmt.Errorf("failed to mark synced: %w", err)
	}

	RecordsSyncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		SyncErrorsCounter.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var query string
		var args []interface{}
		if s.provider.IsSQLite() {
			query = `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES (?, ?, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
			args = []interface{}{r.ID, r.Context}
		} else {
			query = `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`
			args = []interface{}{r.ID, r.Context}
		}

		if _, err := tx.Exec(ctx, query, args...); err != nil {
			SyncErrorsCounter.Add(ctx, 1)
			return fmt.Errorf("failed to insert record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		SyncErrorsCounter.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

var (
	meter                = otel.Meter("rag-sync")
	RecordsSyncedCounter metric.Int64Counter
	SyncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	RecordsSyncedCounter, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced successfully"),
	)
	if err != nil {
		panic(err)
	}

	SyncErrorsCounter, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		panic(err)
	}
}
