package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
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
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSyncedCounter metric.Int64Counter
	syncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	recordsSyncedCounter, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	syncErrorsCounter, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

type DefaultRAGSyncService struct {
	db *sql.DB
}

func NewDefaultRAGSyncService(db *sql.DB) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorStr sql.NullString
		var lastSync sql.NullTime

		if err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSync); err != nil {
			return nil, err
		}

		if vectorStr.Valid && vectorStr.String != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(vectorStr.String), &vec); err == nil {
				r.Vector = vec
			}
		}

		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}

		records = append(records, r)
	}

	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	now := time.Now().UTC()
	for _, id := range ids {
		_, err := tx.ExecContext(ctx, `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = $1
			WHERE memory_id = $2
		`, now, id)
		if err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return err
	}

	recordsSyncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	for _, r := range records {
		var vecStr *string
		if r.Vector != nil {
			b, err := json.Marshal(r.Vector)
			if err != nil {
				syncErrorsCounter.Add(ctx, 1)
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
			str := string(b)
			vecStr = &str
		}

		_, err = tx.ExecContext(ctx, `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT(memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_at = excluded.last_sync_at
		`, r.ID, r.Context, vecStr, r.LastSyncAt)

		if err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return err
	}

	recordsSyncedCounter.Add(ctx, int64(len(records)))

	return nil
}
