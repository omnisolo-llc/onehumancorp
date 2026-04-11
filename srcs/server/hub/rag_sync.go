package hub

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
	syncRecordsCounter metric.Int64Counter
	syncErrorsCounter  metric.Int64Counter
)

func init() {
	meter := otel.Meter("hub")
	syncRecordsCounter, _ = meter.Int64Counter("rag_records_synced_total")
	syncErrorsCounter, _ = meter.Int64Counter("rag_sync_errors_total")
}

type ragSyncService struct {
	db db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		db: provider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_timestamp FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var syncStatus string
		var vectorStr *string
		var lastSyncAt *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &syncStatus, &lastSyncAt); err != nil {
			return nil, err
		}

		if vectorStr != nil {
			var vec []float32
			if err := json.Unmarshal([]byte(*vectorStr), &vec); err == nil {
				rec.Vector = vec
			}
		}

		rec.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP WHERE id = $1`
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			if syncErrorsCounter != nil {
				syncErrorsCounter.Add(ctx, 1)
			}
			return err
		}
		if syncRecordsCounter != nil {
			syncRecordsCounter.Add(ctx, 1)
		}
	}
	return tx.Commit(ctx)
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var query string
		if s.db.IsSQLite() {
			query = `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
				VALUES ($1, $2, CAST($3 AS TEXT), $4, $5)
				ON CONFLICT (id) DO UPDATE SET
				content = excluded.content,
				embedding = excluded.embedding,
				sync_status = excluded.sync_status,
				last_sync_timestamp = excluded.last_sync_timestamp`
		} else {
			query = `INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_timestamp)
				VALUES ($1, $2, $3::vector, $4, $5)
				ON CONFLICT (id) DO UPDATE SET
				content = excluded.content,
				embedding = excluded.embedding,
				sync_status = excluded.sync_status,
				last_sync_timestamp = excluded.last_sync_timestamp`
		}

		vectorBytes, _ := json.Marshal(rec.Vector)
		vectorStr := string(vectorBytes)
		if vectorStr == "null" {
			vectorStr = "[]"
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorStr, string(rec.SyncStatus), rec.LastSyncAt)
		if err != nil {
			if syncErrorsCounter != nil {
				syncErrorsCounter.Add(ctx, 1)
			}
			return err
		}
	}
	return tx.Commit(ctx)
}
