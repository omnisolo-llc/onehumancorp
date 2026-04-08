package hub

import (
	"context"
	"fmt"
	"strings"
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

type HybridRAGSyncService struct {
	dbProvider   db.Provider
	syncedTotal  metric.Int64Counter
	errorsTotal  metric.Int64Counter
}

func NewHybridRAGSyncService(dbProvider db.Provider) (*HybridRAGSyncService, error) {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

	syncedTotal, err := meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create syncedTotal metric: %w", err)
	}

	errorsTotal, err := meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create errorsTotal metric: %w", err)
	}

	return &HybridRAGSyncService{
		dbProvider:  dbProvider,
		syncedTotal: syncedTotal,
		errorsTotal: errorsTotal,
	}, nil
}

func (s *HybridRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt *time.Time
		if err := rows.Scan(&record.ID, &record.Context, &record.SyncStatus, &lastSyncAt); err != nil {
			s.errorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		if lastSyncAt != nil {
			record.LastSyncAt = *lastSyncAt
		}
		records = append(records, record)
	}
	if err := rows.Err(); err != nil {
		s.errorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *HybridRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	query := `
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE id IN (`

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}
	query += strings.Join(placeholders, ",") + ")"

	_, err := s.dbProvider.Exec(ctx, query, args...)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to mark records as synced: %w", err)
	}

	s.syncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *HybridRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	isSQLite := s.dbProvider.IsSQLite()

	for _, record := range records {
		var query string
		if isSQLite {
			query = `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
				VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET
					content = excluded.content,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
		} else {
			query = `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
				VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (id) DO UPDATE SET
					content = EXCLUDED.content,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
		}

		_, err := tx.Exec(ctx, query, record.ID, record.Context)
		if err != nil {
			s.errorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for record %s: %w", record.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		s.errorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit incoming sync transaction: %w", err)
	}

	s.syncedTotal.Add(ctx, int64(len(records)))
	return nil
}
