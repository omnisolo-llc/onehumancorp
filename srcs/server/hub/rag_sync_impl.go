package hub

import (
	"context"
    "database/sql"
	"time"
	"encoding/json"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// RAGSyncServiceImpl provides a concrete implementation of RAGSyncService.
type DefaultRAGSyncService struct {
	db db.Provider
}

// NewDefaultRAGSyncService creates a new DefaultRAGSyncService.
func NewDefaultRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &DefaultRAGSyncService{db: dbProvider}
}

// FetchPendingSyncs fetches records from the database that have a 'pending' status.
func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		if SyncErrorsCounter != nil {
			SyncErrorsCounter.Add(ctx, 1)
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
        var syncStatus string
        var lastSyncAt sql.NullString
        var vectorBytes []byte
		if err := rows.Scan(&r.ID, &r.Context, &vectorBytes, &syncStatus, &lastSyncAt); err != nil {
            if SyncErrorsCounter != nil {
                SyncErrorsCounter.Add(ctx, 1)
            }
			return nil, err
		}
        if len(vectorBytes) > 0 {
            json.Unmarshal(vectorBytes, &r.Vector)
        }
        r.SyncStatus = SyncStatus(syncStatus)
        if lastSyncAt.Valid {
            t, err := time.Parse("2006-01-02 15:04:05", lastSyncAt.String)
            if err == nil {
                r.LastSyncAt = t
            } else {
                t, err := time.Parse(time.RFC3339, lastSyncAt.String)
                if err == nil {
                    r.LastSyncAt = t
                }
            }
        }
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
        if SyncErrorsCounter != nil {
            SyncErrorsCounter.Add(ctx, 1)
        }
		return nil, err
	}

	return records, nil
}

// MarkSynced updates the sync_status of the provided IDs to 'synced'.
func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

    tx, err := s.db.Begin(ctx)
    if err != nil {
        if SyncErrorsCounter != nil {
			SyncErrorsCounter.Add(ctx, 1)
		}
        return err
    }
    defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = $1
	`
	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			if SyncErrorsCounter != nil {
				SyncErrorsCounter.Add(ctx, 1)
			}
			return err
		}
	}

    if err := tx.Commit(ctx); err != nil {
        if SyncErrorsCounter != nil {
			SyncErrorsCounter.Add(ctx, 1)
		}
        return err
    }

	if RecordsSyncedCounter != nil {
		RecordsSyncedCounter.Add(ctx, int64(len(ids)))
	}
	return nil
}

// ProcessIncomingSync processes records incoming from a standalone client.
func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

    tx, err := s.db.Begin(ctx)
    if err != nil {
        if SyncErrorsCounter != nil {
			SyncErrorsCounter.Add(ctx, 1)
		}
        return err
    }
    defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT(memory_id) DO UPDATE SET
			context = EXCLUDED.context,
            vector_embedding = EXCLUDED.vector_embedding,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
	`
	for _, r := range records {
		vectorBytes, _ := json.Marshal(r.Vector)
        _, err := tx.Exec(ctx, query, r.ID, r.Context, vectorBytes)
		if err != nil {
			if SyncErrorsCounter != nil {
				SyncErrorsCounter.Add(ctx, 1)
			}
			return err
		}
	}

    if err := tx.Commit(ctx); err != nil {
        if SyncErrorsCounter != nil {
			SyncErrorsCounter.Add(ctx, 1)
		}
        return err
    }

	if RecordsSyncedCounter != nil {
		RecordsSyncedCounter.Add(ctx, int64(len(records)))
	}
	return nil
}
