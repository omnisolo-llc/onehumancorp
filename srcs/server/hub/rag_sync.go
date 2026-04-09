package hub

import (
	"context"
	"time"
	"database/sql"

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

type RAGSyncMetrics struct {
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
}

func NewRAGSyncMetrics() (*RAGSyncMetrics, error) {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

	recordsSyncedTotal, err := meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced successfully"),
	)
	if err != nil {
		return nil, err
	}

	syncErrorsTotal, err := meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		return nil, err
	}

	return &RAGSyncMetrics{
		RecordsSyncedTotal: recordsSyncedTotal,
		SyncErrorsTotal:    syncErrorsTotal,
	}, nil
}

type DefaultRAGSyncService struct {
	db *sql.DB
}

func NewDefaultRAGSyncService(db *sql.DB) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if s.db == nil {
		return []RAGSyncRecord{}, nil
	}

	rows, err := s.db.QueryContext(ctx, "SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSync); err != nil {
			return nil, err
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if s.db == nil || len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2")
	if err != nil {
		return err
	}
	defer stmt.Close()

	now := time.Now()
	for _, id := range ids {
		if _, err := stmt.ExecContext(ctx, now, id); err != nil {
			return err
		}
	}

	return tx.Commit()
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if s.db == nil || len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ($1, $2, $3, $4) ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at")
	if err != nil {
		return err
	}
	defer stmt.Close()

	for _, r := range records {
		if _, err := stmt.ExecContext(ctx, r.ID, r.Context, string(r.SyncStatus), r.LastSyncAt); err != nil {
			return err
		}
	}

	return tx.Commit()
}
