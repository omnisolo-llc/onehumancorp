package hub

import (
	"context"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"

	"github.com/onehumancorp/mono/srcs/server/db"
)

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

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []byte
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
	db *db.DB
}

func NewRAGSyncService(db *db.DB) RAGSyncService {
	return &ragSyncServiceImpl{
		db: db,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_timestamp
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		syncErrorsCounter.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var syncStatus string
		var lastSyncAt *time.Time
		if err := rows.Scan(&record.ID, &record.Context, &record.Vector, &syncStatus, &lastSyncAt); err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return nil, err
		}
		record.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}
		records = append(records, record)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Simplistic batch update, in production a more robust approach using UNNEST or similar might be used
	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_timestamp = $1
			WHERE memory_id = $2
		`
		_, err := s.db.Exec(ctx, query, time.Now(), id)
		if err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}
	recordsSyncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, record := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_timestamp)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT(memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_timestamp = EXCLUDED.last_sync_timestamp
		`
		_, err := s.db.Exec(ctx, query, record.ID, record.Context, record.Vector, time.Now())
		if err != nil {
			syncErrorsCounter.Add(ctx, 1)
			return err
		}
	}
	recordsSyncedCounter.Add(ctx, int64(len(records)))
	return nil
}
