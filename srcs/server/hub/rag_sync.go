package hub

import (
	"context"
	"database/sql"
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

type DBRAGSyncService struct {
	db               db.Provider
	syncedCounter    metric.Int64Counter
	syncErrorCounter metric.Int64Counter
}

func NewDBRAGSyncService(provider db.Provider) *DBRAGSyncService {
	meter := otel.Meter("hub")
	syncedCounter, _ := meter.Int64Counter("rag_records_synced_total")
	syncErrorCounter, _ := meter.Int64Counter("rag_sync_errors_total")

	return &DBRAGSyncService{
		db:               provider,
		syncedCounter:    syncedCounter,
		syncErrorCounter: syncErrorCounter,
	}
}

func (s *DBRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	if s.db.IsSQLite() {
		query = `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?`
	}

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		s.syncErrorCounter.Add(ctx, 1)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			s.syncErrorCounter.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		s.syncErrorCounter.Add(ctx, 1)
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *DBRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	if s.db.IsSQLite() {
		// SQLite doesn't support ANY(), so we use IN (...)
		placeholders := make([]string, len(ids))
		args := make([]interface{}, len(ids))
		for i, id := range ids {
			placeholders[i] = "?"
			args[i] = id
		}
		query := fmt.Sprintf(`UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN (%s)`, strings.Join(placeholders, ","))
		affected, err := s.db.Exec(ctx, query, args...)
		if err != nil {
			s.syncErrorCounter.Add(ctx, 1)
			return fmt.Errorf("failed to mark synced in sqlite: %w", err)
		}
		s.syncedCounter.Add(ctx, affected)
		return nil
	}

	query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = ANY($1)`
	affected, err := s.db.Exec(ctx, query, ids)
	if err != nil {
		s.syncErrorCounter.Add(ctx, 1)
		return fmt.Errorf("failed to mark synced in postgres: %w", err)
	}
	s.syncedCounter.Add(ctx, affected)
	return nil
}

func (s *DBRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		s.syncErrorCounter.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	upsertQuery := `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET content = EXCLUDED.content, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
	if s.db.IsSQLite() {
		upsertQuery = `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES (?, ?, 'synced', CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET content = excluded.content, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
	}

	var synced int64
	for _, r := range records {
		_, err := tx.Exec(ctx, upsertQuery, r.ID, r.Context)
		if err != nil {
			s.syncErrorCounter.Add(ctx, 1)
			return fmt.Errorf("failed to upsert incoming record: %w", err)
		}
		synced++
	}

	if err := tx.Commit(ctx); err != nil {
		s.syncErrorCounter.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	s.syncedCounter.Add(ctx, synced)
	return nil
}
