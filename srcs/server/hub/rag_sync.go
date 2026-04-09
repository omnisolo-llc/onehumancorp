package hub

import (
	"context"
	"encoding/json"
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
	OrgID      string
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, organization_id, content, embedding, sync_status, last_sync_at FROM consolidated_memory WHERE sync_status = $1 LIMIT $2`
	rows, err := s.db.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingJSON *string
		var status *string
		var lastSyncAt *time.Time

		if err := rows.Scan(&r.ID, &r.OrgID, &r.Context, &embeddingJSON, &status, &lastSyncAt); err != nil {
			return nil, err
		}

		if status != nil {
			r.SyncStatus = SyncStatus(*status)
		} else {
			r.SyncStatus = SyncStatusPending
		}

		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}

		if embeddingJSON != nil && *embeddingJSON != "" {
			_ = json.Unmarshal([]byte(*embeddingJSON), &r.Vector)
		}

		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, `UPDATE consolidated_memory SET sync_status = $1, last_sync_at = $2 WHERE id = $3`, string(SyncStatusSynced), now, id)
		if err != nil {
			return err
		}
	}

	err = tx.Commit(ctx)
	if err == nil {
		RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return err
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var embeddingArg any
		if len(r.Vector) > 0 {
			embeddingJSON, _ := json.Marshal(r.Vector)
			embeddingArg = string(embeddingJSON)
		} else {
			embeddingArg = nil
		}

		query := `
            INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT(id) DO UPDATE SET
                content = excluded.content,
                embedding = excluded.embedding,
                sync_status = excluded.sync_status,
                last_sync_at = excluded.last_sync_at
        `
		_, err := tx.Exec(ctx, query, r.ID, r.OrgID, r.Context, embeddingArg, "sync", string(SyncStatusSynced), time.Now())
		if err != nil {
			return err
		}
	}

	err = tx.Commit(ctx)
	if err == nil {
		RecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return err
}

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
	SyncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
)
