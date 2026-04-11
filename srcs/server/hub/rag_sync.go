package hub

import (
	"context"
	"database/sql"
	"time"

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
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter                 = otel.Meter("ohc/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced to the cloud"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

type SQLRAGSyncService struct {
	db *sql.DB
}

func NewSQLRAGSyncService(db *sql.DB) *SQLRAGSyncService {
	return &SQLRAGSyncService{db: db}
}

func (s *SQLRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *SQLRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create a transaction
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, id := range ids {
		_, err := tx.ExecContext(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2", now, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit()
	if err == nil {
		ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return err
}

func (s *SQLRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, r := range records {
		_, err = tx.ExecContext(ctx, "INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, $3, $4) ON CONFLICT(id) DO UPDATE SET content = $2, sync_status = $3, last_sync_at = $4", r.ID, r.Context, SyncStatusSynced, now)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit()
	if err == nil {
		ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return err
}
