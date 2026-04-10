package hub

import (
	"context"
	"time"
	"fmt"
	"log/slog"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric"
)

var (
	RecordsSynced metric.Int64Counter
	SyncErrors    metric.Int64Counter
)

type mockableMeter interface {
	Int64Counter(name string, options ...metric.Int64CounterOption) (metric.Int64Counter, error)
}

func InitMetrics(m mockableMeter) error {
	var err error
	RecordsSynced, err = m.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced to cloud"))
	if err != nil {
		return err
	}
	SyncErrors, err = m.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
	return err
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

type DefaultRAGSyncService struct {
	db db.Provider
}

func NewDefaultRAGSyncService(database db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: database}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, "SELECT id, content, sync_status FROM autodream_memories WHERE sync_status = $1 LIMIT $2", string(SyncStatusPending), limit)
	if err != nil {
		if SyncErrors != nil {
			SyncErrors.Add(ctx, 1)
		}
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var syncStatus string
		if err := rows.Scan(&r.ID, &r.Context, &syncStatus); err != nil {
			if SyncErrors != nil {
				SyncErrors.Add(ctx, 1)
			}
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		r.SyncStatus = SyncStatus(syncStatus)
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		if SyncErrors != nil {
			SyncErrors.Add(ctx, 1)
		}
		return nil, fmt.Errorf("error iterating pending syncs: %w", err)
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		if SyncErrors != nil {
			SyncErrors.Add(ctx, 1)
		}
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE autodream_memories SET sync_status = $1, last_sync_at = CURRENT_TIMESTAMP WHERE id = $2", string(SyncStatusSynced), id)
		if err != nil {
			if SyncErrors != nil {
				SyncErrors.Add(ctx, 1)
			}
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if SyncErrors != nil {
			SyncErrors.Add(ctx, 1)
		}
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if RecordsSynced != nil {
		RecordsSynced.Add(ctx, int64(len(ids)))
	}
	slog.Info("RAG records marked as synced", "count", len(ids))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		if SyncErrors != nil {
			SyncErrors.Add(ctx, 1)
		}
		return fmt.Errorf("failed to begin transaction for incoming sync: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		_, err := tx.Exec(ctx, `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`, r.ID, r.Context, string(SyncStatusSynced))

		if err != nil {
			if SyncErrors != nil {
				SyncErrors.Add(ctx, 1)
			}
			return fmt.Errorf("failed to insert/update record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if SyncErrors != nil {
			SyncErrors.Add(ctx, 1)
		}
		return fmt.Errorf("failed to commit incoming sync: %w", err)
	}

	if RecordsSynced != nil {
		RecordsSynced.Add(ctx, int64(len(records)))
	}
	slog.Info("Incoming RAG records processed", "count", len(records))
	return nil
}
