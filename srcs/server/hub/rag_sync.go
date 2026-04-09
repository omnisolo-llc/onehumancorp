package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSyncedCounter metric.Int64Counter
	RagSyncErrorsCounter    metric.Int64Counter
)

func init() {
	var err error
	RagRecordsSyncedCounter, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	RagSyncErrorsCounter, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG synchronization"),
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
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) RAGSyncService {
	return &ragSyncServiceImpl{db: db}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT id, context, vector, sync_status, last_sync_at FROM rag_memories WHERE sync_status = $1 LIMIT $2", SyncStatusPending, limit)
	if err != nil {
		RagSyncErrorsCounter.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vecStr string
		var lastSync sql.NullTime
		if err := rows.Scan(&rec.ID, &rec.Context, &vecStr, &rec.SyncStatus, &lastSync); err != nil {
			RagSyncErrorsCounter.Add(ctx, 1)
			return nil, err
		}
		if vecStr != "" {
			json.Unmarshal([]byte(vecStr), &rec.Vector)
		}
		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		RagSyncErrorsCounter.Add(ctx, 1)
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, id := range ids {
		_, err := tx.ExecContext(ctx, "UPDATE rag_memories SET sync_status = $1, last_sync_at = $2 WHERE id = $3", SyncStatusSynced, now, id)
		if err != nil {
			RagSyncErrorsCounter.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		RagSyncErrorsCounter.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		RagSyncErrorsCounter.Add(ctx, 1)
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, rec := range records {
		var vecStr string
		if len(rec.Vector) > 0 {
			b, _ := json.Marshal(rec.Vector)
			vecStr = string(b)
		}

		_, err := tx.ExecContext(ctx, `
			INSERT INTO rag_memories (id, context, vector, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT(id) DO UPDATE SET context = $2, vector = $3, sync_status = $4, last_sync_at = $5`,
			rec.ID, rec.Context, vecStr, SyncStatusSynced, now)
		if err != nil {
			RagSyncErrorsCounter.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(); err != nil {
		RagSyncErrorsCounter.Add(ctx, 1)
		return err
	}

	return nil
}
