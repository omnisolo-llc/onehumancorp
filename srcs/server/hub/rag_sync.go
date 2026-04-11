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

type SqliteRAGSyncService struct {
	provider          db.Provider
	syncedTotalMetric metric.Int64Counter
	errorsTotalMetric metric.Int64Counter
}

func NewSqliteRAGSyncService(provider db.Provider) (*SqliteRAGSyncService, error) {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

	syncedTotal, err := meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		return nil, err
	}

	errorsTotal, err := meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		return nil, err
	}

	return &SqliteRAGSyncService{
		provider:          provider,
		syncedTotalMetric: syncedTotal,
		errorsTotalMetric: errorsTotal,
	}, nil
}

func (s *SqliteRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		s.errorsTotalMetric.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingStr *string
		var lastSyncAt *time.Time

		if err := rows.Scan(&r.ID, &r.Context, &embeddingStr, &r.SyncStatus, &lastSyncAt); err != nil {
			s.errorsTotalMetric.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if embeddingStr != nil {
			if err := json.Unmarshal([]byte(*embeddingStr), &r.Vector); err != nil {
				// Non-fatal, just log metric or continue
			}
		}

		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}

		records = append(records, r)
	}

	return records, nil
}

func (s *SqliteRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		s.errorsTotalMetric.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2", now, id)
		if err != nil {
			s.errorsTotalMetric.Add(ctx, 1)
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		s.errorsTotalMetric.Add(ctx, 1)
		return err
	}

	s.syncedTotalMetric.Add(ctx, int64(len(ids)))
	return nil
}

func (s *SqliteRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		s.errorsTotalMetric.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	for _, record := range records {
		var embeddingStr *string
		if record.Vector != nil {
			b, err := json.Marshal(record.Vector)
			if err == nil {
				s := string(b)
				embeddingStr = &s
			}
		}

		if s.provider.IsSQLite() {
			query := `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, CAST($3 AS TEXT), $4, $5)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
			_, err = tx.Exec(ctx, query, record.ID, record.Context, embeddingStr, "synced", record.LastSyncAt)
		} else {
			query := `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3::vector, $4, $5)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
			_, err = tx.Exec(ctx, query, record.ID, record.Context, embeddingStr, "synced", record.LastSyncAt)
		}

		if err != nil {
			s.errorsTotalMetric.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", record.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		s.errorsTotalMetric.Add(ctx, 1)
		return err
	}

	return nil
}
