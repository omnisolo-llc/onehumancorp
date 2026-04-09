package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"
)

type ragSyncServiceImpl struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) RAGSyncService {
	return &ragSyncServiceImpl{db: db}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorJSON []byte
		var lastSync interface{}

		if err := rows.Scan(&r.ID, &r.Context, &vectorJSON, &r.SyncStatus, &lastSync); err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return nil, err
		}

		if len(vectorJSON) > 0 {
			if err := json.Unmarshal(vectorJSON, &r.Vector); err != nil {
				SyncErrorsTotal.Add(ctx, 1)
				return nil, err
			}
		}

		if lastSync != nil {
			switch v := lastSync.(type) {
			case time.Time:
				r.LastSyncAt = v
			case string:
				t, err := time.Parse("2006-01-02 15:04:05.999999999-07:00", v)
				if err == nil {
					r.LastSyncAt = t
				} else {
					t, err = time.Parse(time.RFC3339, v)
					if err == nil {
						r.LastSyncAt = t
					}
				}
			case []byte:
				t, err := time.Parse("2006-01-02 15:04:05.999999999-07:00", string(v))
				if err == nil {
					r.LastSyncAt = t
				} else {
					t, err = time.Parse(time.RFC3339, string(v))
					if err == nil {
						r.LastSyncAt = t
					}
				}
			}
		}

		records = append(records, r)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}
	query := fmt.Sprintf(`UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (%s)`, strings.Join(placeholders, ","))
	_, err := s.db.ExecContext(ctx, query, args...)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}
	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback()

	for _, r := range records {
		vectorJSON, _ := json.Marshal(r.Vector)
		query := `
            INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
            VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
            ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, vector_embedding=EXCLUDED.vector_embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP
        `
		_, err := tx.ExecContext(ctx, query, r.ID, r.Context, vectorJSON, SyncStatusSynced)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}
	if err := tx.Commit(); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}
	RecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
