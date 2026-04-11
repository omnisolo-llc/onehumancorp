package hub

import (
	"context"
	"time"
	"encoding/json"

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
	LastSyncAt *time.Time
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
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics(meter metric.Meter) {
	if meter == nil {
		return
	}
	ragRecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	ragSyncErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total")
}

func RecordRAGRecordsSynced(ctx context.Context, count int64) {
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, count)
	}
}

func RecordRAGSyncError(ctx context.Context) {
	if ragSyncErrorsTotal != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
	}
}

type RAGSyncServiceImpl struct {
	db *db.DB
}

func NewRAGSyncService(database *db.DB) RAGSyncService {
	return &RAGSyncServiceImpl{db: database}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingStr *string
		if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &r.SyncStatus, &r.LastSyncAt); err != nil {
			return nil, err
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
	for _, id := range ids {
		_, err := s.db.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1", id)
		if err != nil {
			return err
		}
	}
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		var embeddingStr *string
		if r.Vector != nil {
			b, _ := json.Marshal(r.Vector)
			s := string(b)
			embeddingStr = &s
		}

		_, err := s.db.Exec(ctx, `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CASE WHEN $3::text IS NULL THEN NULL ELSE $3::text::vector END, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at`,
			r.ID, r.Context, embeddingStr, r.SyncStatus, r.LastSyncAt)
		if err != nil {
			return err
		}
	}
	return nil
}
