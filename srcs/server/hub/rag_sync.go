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
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		panic(err)
	}
}

type DB_RAGSyncService struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &DB_RAGSyncService{dbProvider: dbProvider}
}

func (s *DB_RAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	q := `SELECT id, content, sync_status FROM consolidated_memory WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.dbProvider.Query(ctx, q, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus)
		if err != nil {
			return nil, err
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *DB_RAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	if s.dbProvider.IsSQLite() {
		for _, id := range ids {
			_, err = tx.Exec(ctx, `UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`, id)
			if err != nil {
				return err
			}
		}
	} else {
		// PostgreSQL array update logic not fully robust across abstraction yet
		for _, id := range ids {
			_, err = tx.Exec(ctx, `UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`, id)
			if err != nil {
				return err
			}
		}
	}
	return tx.Commit(ctx)
}

func (s *DB_RAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		if s.dbProvider.IsSQLite() {
			_, err = tx.Exec(ctx, `INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status, last_sync_at) VALUES ($1, 'system', $2, 'hybrid_sync', $3, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, sync_status = EXCLUDED.sync_status, last_sync_at = CURRENT_TIMESTAMP`, r.ID, r.Context, SyncStatusSynced)
		} else {
			_, err = tx.Exec(ctx, `INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status, last_sync_at) VALUES ($1, 'system', $2, 'hybrid_sync', $3, CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, sync_status = EXCLUDED.sync_status, last_sync_at = CURRENT_TIMESTAMP`, r.ID, r.Context, SyncStatusSynced)
		}
		if err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}
