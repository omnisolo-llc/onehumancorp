package hub

import (
	"context"
	"encoding/json"
	"fmt"
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

type RAGSyncServiceImpl struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &RAGSyncServiceImpl{db: db}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending' OR sync_status IS NULL
		LIMIT $1
	`, limit)
	if err != nil {
		RecordSyncError(ctx)
		return nil, fmt.Errorf("querying pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingStr *string
		var syncStatusStr *string
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &syncStatusStr, &lastSyncAt); err != nil {
			RecordSyncError(ctx)
			return nil, fmt.Errorf("scanning record: %w", err)
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		if syncStatusStr != nil && *syncStatusStr != "" {
			r.SyncStatus = SyncStatus(*syncStatusStr)
		} else {
			r.SyncStatus = SyncStatusPending
		}
		if embeddingStr != nil && *embeddingStr != "" {
			if err := json.Unmarshal([]byte(*embeddingStr), &r.Vector); err != nil {
				RecordSyncError(ctx)
				return nil, fmt.Errorf("unmarshaling vector: %w", err)
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

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE id = $1
		`, id)
		if err != nil {
			RecordSyncError(ctx)
			return fmt.Errorf("updating record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RecordSyncError(ctx)
		return fmt.Errorf("commit transaction: %w", err)
	}

	RecordSyncSuccess(ctx, int64(len(ids)))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var embeddingStr *string
		if r.Vector != nil {
			b, err := json.Marshal(r.Vector)
			if err != nil {
				RecordSyncError(ctx)
				return fmt.Errorf("marshaling vector: %w", err)
			}
			str := string(b)
			embeddingStr = &str
		}

		var args []interface{}
		var query string

		if s.db.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
			args = []interface{}{r.ID, r.Context, embeddingStr}
		} else {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3::vector, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
			args = []interface{}{r.ID, r.Context, embeddingStr}
		}

		_, err := tx.Exec(ctx, query, args...)
		if err != nil {
			RecordSyncError(ctx)
			return fmt.Errorf("upserting record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RecordSyncError(ctx)
		return fmt.Errorf("commit transaction: %w", err)
	}

	RecordSyncSuccess(ctx, int64(len(records)))
	return nil
}

var (
	meter                 metric.Meter
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	meter = otel.Meter("hub_rag_sync")
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		fmt.Printf("failed to initialize rag_records_synced_total: %v\n", err)
	}
	ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		fmt.Printf("failed to initialize rag_sync_errors_total: %v\n", err)
	}
}

func RecordSyncSuccess(ctx context.Context, count int64) {
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, count)
	}
}

func RecordSyncError(ctx context.Context) {
	if ragSyncErrorsTotal != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
	}
}
