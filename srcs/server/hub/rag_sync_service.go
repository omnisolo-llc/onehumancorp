package hub

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ragSyncService struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) RAGSyncService {
	return &ragSyncService{
		db: db,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM agent_memories
		WHERE sync_status = $1
		LIMIT $2`

	rows, err := s.db.QueryContext(ctx, query, SyncStatusPending, limit)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&record.ID, &record.Context, &record.SyncStatus, &lastSyncAt); err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		if lastSyncAt.Valid {
			record.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, record)
	}

	if err := rows.Err(); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, fmt.Errorf("error iterating pending syncs: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	// Use standard parameterized updates, doing one by one for cross-compatibility
	// between SQLite and PostgreSQL, or you could build a batch query.
	stmt, err := tx.PrepareContext(ctx, `
		UPDATE agent_memories
		SET sync_status = $1, last_sync_at = $2
		WHERE id = $3`)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to prepare statement: %w", err)
	}
	defer stmt.Close()

	now := time.Now()
	for _, id := range ids {
		_, err := stmt.ExecContext(ctx, SyncStatusSynced, now, id)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	if err := tx.Commit(); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	// Upsert logic compatible with both Postgres and SQLite
	// Assuming agent_memories has a primary key on id
	stmt, err := tx.PrepareContext(ctx, `
		INSERT INTO agent_memories (id, content, sync_status, last_sync_at, organization_id)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT(id) DO UPDATE SET
			content = excluded.content,
			sync_status = excluded.sync_status,
			last_sync_at = excluded.last_sync_at
	`)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to prepare statement: %w", err)
	}
	defer stmt.Close()

	now := time.Now()
	for _, record := range records {
		// Mock organization ID for incoming syncs from standalone
		orgID := "standalone-sync"
		_, err := stmt.ExecContext(ctx, record.ID, record.Context, SyncStatusSynced, now, orgID)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to upsert record %s: %w", record.ID, err)
		}
	}

	if err := tx.Commit(); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}

	return nil
}
