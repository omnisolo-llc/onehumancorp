package hub

import (
	"context"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type defaultRAGSyncService struct {
	dbProvider db.Provider
}

// NewRAGSyncService creates a new RAGSyncService instance
func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &defaultRAGSyncService{
		dbProvider: dbProvider,
	}
}

func (s *defaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending' OR sync_status IS NULL
		ORDER BY created_at ASC
		LIMIT $1
	`
	// SQLite and Postgres parameter placeholders might vary, but db.Provider handles standardizing if built well, or we might need standard SQL
	// $1 is Postgres, SQLite usually uses ? but standard pgx/sqlite wrapper might adapt it.
	if s.dbProvider.IsSQLite() {
		query = strings.ReplaceAll(query, "$1", "?")
	}

	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		slog.ErrorContext(ctx, "failed to query pending syncs", "error", err)
		return nil, fmt.Errorf("query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSync *time.Time
		if err := rows.Scan(&rec.ID, &rec.Context, &rec.SyncStatus, &lastSync); err != nil {
			slog.ErrorContext(ctx, "failed to scan pending sync record", "error", err)
			continue
		}
		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}
		if rec.SyncStatus == "" {
			rec.SyncStatus = SyncStatusPending
		}
		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *defaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Dynamic placeholders
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids)+1)
	now := time.Now()
	args[0] = now

	for i, id := range ids {
		if s.dbProvider.IsSQLite() {
			placeholders[i] = "?"
		} else {
			placeholders[i] = fmt.Sprintf("$%d", i+2)
		}
		args[i+1] = id
	}

	query := fmt.Sprintf(`
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = %s
		WHERE memory_id IN (%s)
	`, func() string {
		if s.dbProvider.IsSQLite() {
			return "?"
		}
		return "$1"
	}(), strings.Join(placeholders, ","))

	_, err := s.dbProvider.Exec(ctx, query, args...)
	if err != nil {
		slog.ErrorContext(ctx, "failed to mark records synced", "error", err)
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("mark synced: %w", err)
	}

	telemetry.RecordRAGSync(ctx, len(ids))
	return nil
}

func (s *defaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	for _, rec := range records {
		var upsertQuery string
		if s.dbProvider.IsSQLite() {
			upsertQuery = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
				VALUES (?, ?, 'synced', ?)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = excluded.context,
					sync_status = 'synced',
					last_sync_at = excluded.last_sync_at
			`
			_, err = tx.Exec(ctx, upsertQuery, rec.ID, rec.Context, time.Now())
		} else {
			upsertQuery = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
				VALUES ($1, $2, 'synced', $3)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = EXCLUDED.context,
					sync_status = 'synced',
					last_sync_at = EXCLUDED.last_sync_at
			`
			_, err = tx.Exec(ctx, upsertQuery, rec.ID, rec.Context, time.Now())
		}

		if err != nil {
			slog.ErrorContext(ctx, "failed to upsert incoming sync record", "id", rec.ID, "error", err)
			telemetry.RecordRAGSyncError(ctx)
			return fmt.Errorf("upsert record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return fmt.Errorf("commit tx: %w", err)
	}

	telemetry.RecordRAGSync(ctx, len(records))
	return nil
}
