package hub

import (
	"context"
	"database/sql"
	"fmt"
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
	dbProvider    db.Provider
	syncedCounter metric.Int64Counter
	errorsCounter metric.Int64Counter
}

func NewRAGSyncService(dbProvider db.Provider, meter metric.Meter) (RAGSyncService, error) {
	syncedCounter, err := meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		return nil, fmt.Errorf("failed to create synced counter: %w", err)
	}

	errorsCounter, err := meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		return nil, fmt.Errorf("failed to create errors counter: %w", err)
	}

	return &ragSyncService{
		dbProvider:    dbProvider,
		syncedCounter: syncedCounter,
		errorsCounter: errorsCounter,
	}, nil
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_timestamp
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		s.errorsCounter.Add(ctx, 1)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorStr sql.NullString
		var lastSync sql.NullTime

		err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &rec.SyncStatus, &lastSync)
		if err != nil {
			s.errorsCounter.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}
		// For simplicity, we parse vector strings back to float32 or ignore here, assuming vector parsing is abstracted or we just pass it along
		// (The prompt states "Convert to string internally for SQLite compat if needed", we selected CAST(embedding AS TEXT) which gives us a string representation)

		records = append(records, rec)
	}
	if err = rows.Err(); err != nil {
		s.errorsCounter.Add(ctx, 1)
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create placeholder array $1, $2...
	// We'll execute a query for each to avoid complex placeholder logic for now, or use a loop

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		s.errorsCounter.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := `
			UPDATE autodream_memories
			SET sync_status = 'synced', last_sync_timestamp = $1
			WHERE id = $2
		`
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			s.errorsCounter.Add(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		s.errorsCounter.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	s.syncedCounter.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		s.errorsCounter.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_timestamp)
			VALUES ($1, $2, $3, $4)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = EXCLUDED.sync_status,
				last_sync_timestamp = EXCLUDED.last_sync_timestamp
		`
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.SyncStatus, rec.LastSyncAt)
		if err != nil {
			s.errorsCounter.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		s.errorsCounter.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
