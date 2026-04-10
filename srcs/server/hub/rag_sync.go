package hub

import (
	"context"
	"time"
	"strings"
	"fmt"

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

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func InitTelemetry(meter metric.Meter) error {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		return err
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		return err
	}
	return nil
}

func RecordRAGSyncSuccess(ctx context.Context, count int) {
	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, int64(count))
	}
}

func RecordRAGSyncError(ctx context.Context) {
	if ragSyncErrorsTotal != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
	}
}

type DB struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &DB{db: db}
}

func (s *DB) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = $1 LIMIT $2"
	rows, err := s.db.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *DB) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	quotedIDs := make([]string, len(ids))
	for i, id := range ids {
		quotedIDs[i] = fmt.Sprintf("'%s'", id)
	}

	query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id IN (%s)", strings.Join(quotedIDs, ","))
	_, err := s.db.Exec(ctx, query, string(SyncStatusSynced), time.Now())
	return err
}

func (s *DB) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		// UPSERT logic based on UPDATE then INSERT
		queryUpdate := "UPDATE swarm_memory_embeddings SET context = $1, sync_status = $2, last_sync_at = $3 WHERE memory_id = $4"
		rowsAffected, err := tx.Exec(ctx, queryUpdate, r.Context, string(SyncStatusSynced), r.LastSyncAt, r.ID)
		if err != nil {
			return err
		}

		if rowsAffected == 0 {
			queryInsert := "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ($1, $2, $3, $4)"
			_, err = tx.Exec(ctx, queryInsert, r.ID, r.Context, string(SyncStatusSynced), r.LastSyncAt)
			if err != nil {
				return err
			}
		}
	}
	return tx.Commit(ctx)
}
