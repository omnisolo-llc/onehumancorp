package hub

import (
	"context"
	"database/sql"
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type SQLRAGSyncService struct {
	provider db.Provider

	recordsSynced metric.Int64Counter
	syncErrors    metric.Int64Counter
}

func NewSQLRAGSyncService(provider db.Provider) (*SQLRAGSyncService, error) {
	meter := otel.Meter("hub_rag_sync")

	synced, err := meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		return nil, err
	}

	errors, err := meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		return nil, err
	}

	return &SQLRAGSyncService{
		provider:      provider,
		recordsSynced: synced,
		syncErrors:    errors,
	}, nil
}

func (s *SQLRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

func (s *SQLRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create query with variable number of parameters
	query := `UPDATE autodream_memories SET sync_status = $1, last_sync_at = $2 WHERE id IN (`
	args := []interface{}{string(SyncStatusSynced), time.Now()}

	for i, id := range ids {
		if i > 0 {
			query += ", "
		}
		query += fmt.Sprintf("$%d", i+3)
		args = append(args, id)
	}
	query += ")"

	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		s.syncErrors.Add(ctx, 1)
		return err
	}

	s.recordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *SQLRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, record := range records {
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err := s.provider.Exec(ctx, query, record.ID, record.Context, string(record.SyncStatus), record.LastSyncAt)
		if err != nil {
			s.syncErrors.Add(ctx, 1)
			return err
		}
	}

	return nil
}
