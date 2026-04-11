package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

// Ensure metric is used to satisfy the compiler if it was unused directly but initialized via otel.Meter
var _ = metric.WithDescription

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
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSynced, _ = meter.Int64Counter("rag_records_synced_total")
	syncErrors, _    = meter.Int64Counter("rag_sync_errors_total")
)

type DefaultRAGSyncService struct {
	db db.Provider
}

func NewDefaultRAGSyncService(db db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: db}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var query string
	if s.db.IsSQLite() {
		query = `SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	} else {
		// PostgreSQL query handles the vector naturally, but we still cast to text to read it universally into string
		query = `SELECT id, content, CAST(embedding AS TEXT), sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	}

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var embeddingStr *string
		var lastSyncAt *time.Time
		var status string

		if err := rows.Scan(&rec.ID, &rec.Context, &embeddingStr, &status, &lastSyncAt); err != nil {
			syncErrors.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}

		rec.SyncStatus = SyncStatus(status)
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}

		if embeddingStr != nil {
			if err := json.Unmarshal([]byte(*embeddingStr), &rec.Vector); err != nil {
				// Failed to unmarshal vector, just continue with empty/nil slice, or error? Let's just set to nil.
				rec.Vector = nil
			}
		}
		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		syncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`
		if _, err := tx.Exec(ctx, query, id); err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	recordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		vectorJSON, err := json.Marshal(rec.Vector)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to marshal vector: %w", err)
		}

		var query string
		if s.db.IsSQLite() {
			query = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT(id) DO UPDATE SET
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
			`
		} else {
			query = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3::vector, $4, $5)
			ON CONFLICT(id) DO UPDATE SET
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
			`
		}

		if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, string(vectorJSON), string(rec.SyncStatus), rec.LastSyncAt); err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	recordsSynced.Add(ctx, int64(len(records)))
	return nil
}
