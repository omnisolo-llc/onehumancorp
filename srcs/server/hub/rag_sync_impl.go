package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/jackc/pgx/v5/pgxpool"
)

type standaloneRAGSyncService struct {
	db      *sql.DB
	metrics *RAGSyncMetrics
}

func NewStandaloneRAGSyncService(db *sql.DB, metrics *RAGSyncMetrics) RAGSyncService {
	return &standaloneRAGSyncService{db: db, metrics: metrics}
}

func (s *standaloneRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT id, content, embedding, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vecStr sql.NullString
		if err := rows.Scan(&r.ID, &r.Context, &vecStr, &r.SyncStatus); err != nil {
			return nil, err
		}
		if vecStr.Valid && vecStr.String != "" {
            var vec []float32
            // Try to parse JSON array of floats if it's stored as string
            if err := json.Unmarshal([]byte(vecStr.String), &vec); err == nil {
                r.Vector = vec
            }
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *standaloneRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// SQLite has variable limits, simple chunking
	chunkSize := 500
	for i := 0; i < len(ids); i += chunkSize {
	    end := i + chunkSize
	    if end > len(ids) {
	        end = len(ids)
	    }
	    chunk := ids[i:end]

	    query := fmt.Sprintf("UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id IN (%s)", placeholders(len(chunk)))
	args := make([]interface{}, len(chunk))
	for j, id := range chunk {
		args[j] = id
	}

	_, err := s.db.ExecContext(ctx, query, args...)
	if err != nil {
	    if s.metrics != nil {
                s.metrics.SyncErrorsTotal.Add(ctx, 1)
            }
		return err
	}
	}

	if s.metrics != nil {
	    s.metrics.RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *standaloneRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return fmt.Errorf("standalone service cannot process incoming syncs")
}

type cloudRAGSyncService struct {
	pool    *pgxpool.Pool
	metrics *RAGSyncMetrics
}

func NewCloudRAGSyncService(pool *pgxpool.Pool, metrics *RAGSyncMetrics) RAGSyncService {
	return &cloudRAGSyncService{pool: pool, metrics: metrics}
}

func (s *cloudRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return nil, fmt.Errorf("cloud service cannot fetch pending syncs")
}

func (s *cloudRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	return fmt.Errorf("cloud service cannot mark synced")
}

func (s *cloudRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.pool.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
        var vecVal interface{}
        if len(r.Vector) > 0 {
            vecBytes, err := json.Marshal(r.Vector)
            if err != nil {
                if s.metrics != nil {
                    s.metrics.SyncErrorsTotal.Add(ctx, 1)
                }
                return fmt.Errorf("failed to marshal vector: %w", err)
            }
            vecVal = string(vecBytes)
        } else {
            vecVal = nil
        }

		_, err = tx.Exec(ctx, `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`, r.ID, r.Context, vecVal)
		if err != nil {
		    if s.metrics != nil {
                s.metrics.SyncErrorsTotal.Add(ctx, 1)
            }
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
	    if s.metrics != nil {
            s.metrics.SyncErrorsTotal.Add(ctx, 1)
        }
	    return err
	}

	if s.metrics != nil {
	    s.metrics.RecordsSyncedTotal.Add(ctx, int64(len(records)))
	}

	return nil
}

func placeholders(n int) string {
	ps := make([]string, n)
	for i := range ps {
		ps[i] = fmt.Sprintf("$%d", i+1)
	}
	return strings.Join(ps, ",")
}
