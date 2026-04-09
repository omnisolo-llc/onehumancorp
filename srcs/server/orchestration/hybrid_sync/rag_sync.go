package hybrid_sync

import (
	"context"
	"encoding/json"
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
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type RAGSyncServiceImpl struct {
	dbWrapper *db.DB
}

func NewRAGSyncService(dbWrapper *db.DB) RAGSyncService {
	return &RAGSyncServiceImpl{dbWrapper: dbWrapper}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.dbWrapper.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var embeddingStr *string
		if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		if embeddingStr != nil {
			var vec []float32
			if err := json.Unmarshal([]byte(*embeddingStr), &vec); err == nil {
				r.Vector = vec
			}
		}
		records = append(records, r)
	}
	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
		if _, err := tx.Exec(ctx, query, id); err != nil {
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}
	tx, err := s.dbWrapper.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		vecBytes, err := json.Marshal(r.Vector)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			continue
		}

		if s.dbWrapper.IsSQLite() {
			query := "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET content=excluded.content, embedding=excluded.embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP"
			if _, err := tx.Exec(ctx, query, r.ID, r.Context, string(vecBytes)); err != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
				return err
			}
		} else {
			query := "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3::vector, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET content=excluded.content, embedding=excluded.embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP"
			if _, err := tx.Exec(ctx, query, r.ID, r.Context, string(vecBytes)); err != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
				return err
			}
		}
	}
	return tx.Commit(ctx)
}

var meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration/hybrid_sync")
var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced successfully"))
	if err != nil {
		panic(err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors encountered during RAG sync"))
	if err != nil {
		panic(err)
	}
}
