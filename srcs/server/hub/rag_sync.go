package hub

import (
	"context"
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

type DatabaseProvider interface {
	QueryContext(ctx context.Context, query string, args ...any) (Rows, error)
	ExecContext(ctx context.Context, query string, args ...any) (Result, error)
}

type Rows interface {
	Next() bool
	Scan(dest ...any) error
	Close() error
	Err() error
}

type Result interface {
	LastInsertId() (int64, error)
	RowsAffected() (int64, error)
}

type DefaultRAGSyncService struct {
	db DatabaseProvider
}

func NewDefaultRAGSyncService(db DatabaseProvider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var vectorStr *string // Usually pgvector stores string like "[0.1, 0.2]"
		if err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSyncAt); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return nil, err
		}

		// In a real app we would decode vectorStr to []float32.
		// For the purpose of this interface proxying, we just ignore decoding errors if it's nil.

		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`
		_, err := s.db.ExecContext(ctx, query, id)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}
		RagRecordsSyncedTotal.Add(ctx, 1)
	}
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, record := range records {
		// Use a basic upsert logic, assuming Last Write Wins if there's a conflict (based on ID)
		// For simplicity we try update, if 0 rows affected, we insert.

		// Encode vector dummy
		vectorStr := "[0.0]"

		updateQuery := `UPDATE autodream_memories SET content = $1, embedding = $2, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $3`
		res, err := s.db.ExecContext(ctx, updateQuery, record.Context, vectorStr, record.ID)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}

		affected, _ := res.RowsAffected()
		if affected == 0 {
			insertQuery := `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)`
			_, err = s.db.ExecContext(ctx, insertQuery, record.ID, record.Context, vectorStr)
			if err != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
				return err
			}
		}
		RagRecordsSyncedTotal.Add(ctx, 1)
	}
	return nil
}

var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSyncedTotal   metric.Int64Counter
	RagSyncErrorsTotal      metric.Int64Counter
)

func init() {
	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	RagSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}
