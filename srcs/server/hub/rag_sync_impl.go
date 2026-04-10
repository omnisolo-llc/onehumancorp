package hub

import (
	"context"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type sqlRAGSyncService struct {
	prov db.Provider
}

func NewRAGSyncService(prov db.Provider) RAGSyncService {
	return &sqlRAGSyncService{prov: prov}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`

	rows, err := s.prov.Query(ctx, query, limit)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		// Note: Embedding vectors are typically stored as pgvector types or stringified JSON arrays in Hybrid Architecture.
		// A full RAG fetch would SELECT embedding from autodream_memories, but scanning vectors requires
		// pgvector-go types which we are deferring to the auto-dream aggregation loop to avoid dependency bloat here.

		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSync); err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, err
		}

		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}

		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}

	return records, nil
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.prov.Begin(ctx)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1`

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		if telemetry.RagRecordsSyncedTotal != nil {
			telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
		}
	}
	return nil
}
func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.prov.Begin(ctx)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	// Note: Embedding vectors aren't upserted directly by this simple sync service yet;
	// they are assumed to be re-calculated by the Cloud aggregation pipeline (AutoDream Worker)
	// once the content hits the cloud database, or added via a separate sync phase.
	upsertQuery := `INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
	                VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
	                ON CONFLICT (id) DO UPDATE
	                SET content = EXCLUDED.content, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`

	for _, r := range records {
		_, err := tx.Exec(ctx, upsertQuery, r.ID, r.Context)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}
