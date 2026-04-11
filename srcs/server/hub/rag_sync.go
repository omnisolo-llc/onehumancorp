package hub

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
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
	Vector     []byte
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	db db.Provider
}

func NewDefaultRAGSyncService(db db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT "
	var q string
	if s.db.IsSQLite() {
		q = query + "?"
	} else {
		q = query + "$1"
	}

	rows, err := s.db.Query(ctx, q, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var status string
		var lastSync *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &status, &lastSync); err != nil {
			return nil, err
		}
		rec.SyncStatus = SyncStatus(status)
		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		if s.db.IsSQLite() {
			placeholders[i] = "?"
		} else {
			placeholders[i] = fmt.Sprintf("$%d", i+1)
		}
		args[i] = id
	}

	query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (%s)", strings.Join(placeholders, ","))

	_, err := s.db.Exec(ctx, query, args...)
	if err != nil {
		RAGSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes())
		return err
	}

	RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)), metric.WithAttributes())
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	var valueStrings []string
	var valueArgs []interface{}

	for i, rec := range records {
		var p1, p2, p3 string
		if s.db.IsSQLite() {
			p1, p2, p3 = "?", "?", "?"
		} else {
			offset := i * 3
			p1 = fmt.Sprintf("$%d", offset+1)
			p2 = fmt.Sprintf("$%d", offset+2)
			p3 = fmt.Sprintf("$%d", offset+3)
		}

		valueStrings = append(valueStrings, fmt.Sprintf("(%s, %s, %s, 'synced', CURRENT_TIMESTAMP)", p1, p2, p3))
		valueArgs = append(valueArgs, rec.ID, rec.Context, rec.Vector)
	}

	query := fmt.Sprintf(`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
VALUES %s ON CONFLICT(memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`, strings.Join(valueStrings, ","))

	_, err := s.db.Exec(ctx, query, valueArgs...)
	if err != nil {
		RAGSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes())
		return err
	}

	RAGRecordsSyncedTotal.Add(ctx, int64(len(records)), metric.WithAttributes())
	return nil
}
