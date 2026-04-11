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

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced successfully"),
	)
	ragSyncErrorsTotal, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
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

type DefaultRAGSyncService struct {
	Provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{Provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := s.Provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus string
		var vectorStr *string

		err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &syncStatus, &lastSyncAt)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		rec.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}

		if vectorStr != nil {
			err = json.Unmarshal([]byte(*vectorStr), &rec.Vector)
			if err != nil {
			    // Try format like "[1.0, 2.0]"
			    continue // ignore invalid vector formatting for now
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("error iterating rows: %w", err)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.Provider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `
			UPDATE autodream_memories
			SET sync_status = $1, last_sync_at = $2
			WHERE id = $3
		`
		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), time.Now(), id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.Provider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var vectorStr *string
		if len(rec.Vector) > 0 {
			b, err := json.Marshal(rec.Vector)
			if err == nil {
				s := string(b)
				vectorStr = &s
			}
		}

		var query string
		var args []any

		if s.Provider.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, CAST($3 AS TEXT), $4, $5)
				ON CONFLICT(id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
			args = []any{rec.ID, rec.Context, vectorStr, string(SyncStatusSynced), time.Now()}
		} else {
			query = `
				INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3::vector, $4, $5)
				ON CONFLICT(id) DO UPDATE SET
					content = EXCLUDED.content,
					embedding = EXCLUDED.embedding,
					sync_status = EXCLUDED.sync_status,
					last_sync_at = EXCLUDED.last_sync_at
			`
			args = []any{rec.ID, rec.Context, vectorStr, string(SyncStatusSynced), time.Now()}
		}

		_, err = tx.Exec(ctx, query, args...)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert incoming sync record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
