package hub

import (
	"context"
	"database/sql"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type RAGSyncProvider struct {
	provider db.Provider
}

func NewRAGSyncProvider(provider db.Provider) *RAGSyncProvider {
	return &RAGSyncProvider{
		provider: provider,
	}
}

func (s *RAGSyncProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = $1
		LIMIT $2
	`
	if s.provider.IsSQLite() {
		query = strings.ReplaceAll(query, "$1", "?")
		query = strings.ReplaceAll(query, "$2", "?")
	}

	rows, err := s.provider.Query(ctx, query, string(SyncStatusPending), limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullString

		err := rows.Scan(
			&rec.ID,
			&rec.Context,
			&rec.Vector,
			&rec.SyncStatus,
			&lastSyncAt,
		)
		if RAGRecordsSyncedTotal != nil {
			RAGRecordsSyncedTotal.Add(ctx, 1)
		}
		if err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if lastSyncAt.Valid && lastSyncAt.String != "" {
			t, err := time.Parse(time.RFC3339, lastSyncAt.String)
			if err == nil {
				rec.LastSyncAt = t

				t, err := time.Parse("2006-01-02 15:04:05-07:00", lastSyncAt.String)
				if err == nil {
					rec.LastSyncAt = t

					t, err := time.Parse("2006-01-02 15:04:05.999999-07:00", lastSyncAt.String)
					if err == nil {
						rec.LastSyncAt = t
					}
				}
			}
		}

		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

func (s *RAGSyncProvider) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	now := time.Now().UTC().Format(time.RFC3339)

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = $1, last_sync_at = $2
			WHERE memory_id = $3
		`
		if s.provider.IsSQLite() {
			query = strings.ReplaceAll(query, "$1", "?")
			query = strings.ReplaceAll(query, "$2", "?")
			query = strings.ReplaceAll(query, "$3", "?")
		}

		_, err := tx.Exec(ctx, query, string(SyncStatusSynced), now, id)
		if RAGRecordsSyncedTotal != nil {
			RAGRecordsSyncedTotal.Add(ctx, 1)
		}
		if err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	return tx.Commit(ctx)
}

func (s *RAGSyncProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC().Format(time.RFC3339)

	for _, rec := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		if s.provider.IsSQLite() {
			query = strings.ReplaceAll(query, "$1", "?")
			query = strings.ReplaceAll(query, "$2", "?")
			query = strings.ReplaceAll(query, "$3", "?")
			query = strings.ReplaceAll(query, "$4", "?")
			query = strings.ReplaceAll(query, "$5", "?")
		}

		status := string(SyncStatusSynced)
		// Cloud might actually set it to synced since it's now in sync.

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, status, now)
		if RAGRecordsSyncedTotal != nil {
			RAGRecordsSyncedTotal.Add(ctx, 1)
		}
		if err != nil {
			if RAGSyncErrorsTotal != nil {
				RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to process incoming sync for id %s: %w", rec.ID, err)
		}
	}

	return tx.Commit(ctx)
}
