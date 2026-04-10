package hub

import (
	"context"
	"database/sql"
	"fmt"
	"log/slog"
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

type ragSyncService struct {
	provider db.Provider
}

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		slog.Error("failed to initialize rag_records_synced_total metric", "error", err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		slog.Error("failed to initialize rag_sync_errors_total metric", "error", err)
	}
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		provider: provider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.provider.Query(ctx, `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`, limit)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var syncStatus string
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&rec.ID, &rec.Context, &syncStatus, &lastSyncAt); err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		rec.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Simple loop implementation since we don't have a reliable IN clause helper in db.Provider
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to begin transaction: %w", err)
	}

	now := time.Now()
	var successCount int64
	for _, id := range ids {
		_, err := tx.Exec(ctx, `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_at = $1
			WHERE id = $2
		`, now, id)
		if err != nil {
			tx.Rollback(ctx)
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
		successCount++
	}

	if err := tx.Commit(ctx); err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, successCount)
	}

	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to begin transaction: %w", err)
	}

	now := time.Now()
	var successCount int64
	for _, rec := range records {
		// Use UPSERT-like logic. Simple approach for cross-DB compatibility.
		// First try to update, if 0 rows affected, insert.
		rowsAffectedRes, err := tx.Exec(ctx, `
			UPDATE autodream_memories
			SET content = $1, sync_status = 'synced', last_sync_at = $2
			WHERE id = $3
		`, rec.Context, now, rec.ID)
		if err != nil {
			tx.Rollback(ctx)
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to update record %s: %w", rec.ID, err)
		}

		if rowsAffectedRes == 0 {
			_, err = tx.Exec(ctx, `
				INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
				VALUES ($1, $2, 'synced', $3)
			`, rec.ID, rec.Context, now)
			if err != nil {
				tx.Rollback(ctx)
				if ragSyncErrorsTotal != nil {
					ragSyncErrorsTotal.Add(ctx, 1)
				}
				return fmt.Errorf("failed to insert record %s: %w", rec.ID, err)
			}
		}
		successCount++
	}

	if err := tx.Commit(ctx); err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, successCount)
	}

	return nil
}
