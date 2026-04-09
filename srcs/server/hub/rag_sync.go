package hub

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

type ragSyncServiceImpl struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{db: db}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	if s.db.IsSQLite() {
		query = `SELECT id, content, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?`
	}

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus); err != nil {
			return nil, err
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Simply update one by one for both sqlite and postgres for this simplified implementation
	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = NOW() WHERE id = $1`
		if s.db.IsSQLite() {
			query = `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ?`
		}

		_, err := s.db.Exec(ctx, query, id)
		if err != nil {
			RecordSyncError(ctx, 1)
			return err
		}
		RecordSyncSuccess(ctx, 1)
	}

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, rec := range records {
		query := `
			INSERT INTO autodream_memories (id, organization_id, agent_id, content, source_type, sync_status, last_sync_at)
			VALUES ($1, 'default', 'default', $2, 'sync', 'synced', NOW())
			ON CONFLICT(id) DO UPDATE SET content = EXCLUDED.content, sync_status = 'synced', last_sync_at = NOW()
		`
		if s.db.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, source_type, sync_status, last_sync_at)
				VALUES (?, 'default', 'default', ?, 'sync', 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET content = EXCLUDED.content, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			`
		}

		_, err := s.db.Exec(ctx, query, rec.ID, rec.Context)
		if err != nil {
			return err
		}
	}
	return nil
}

var (
	meter                = otel.Meter("hub_rag_sync")
	recordsSyncedTotal   metric.Int64Counter
	recordsSyncErrorTotal metric.Int64Counter
)

func init() {
	var err error
	recordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	recordsSyncErrorTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

func RecordSyncSuccess(ctx context.Context, count int64) {
	recordsSyncedTotal.Add(ctx, count)
}

func RecordSyncError(ctx context.Context, count int64) {
	recordsSyncErrorTotal.Add(ctx, count)
}
