package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
)

type defaultRAGSyncService struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) RAGSyncService {
	return &defaultRAGSyncService{db: db}
}

func (s *defaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		SyncErrorsCounter.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSync sql.NullTime
		var vectorBytes []byte
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &rec.SyncStatus, &lastSync); err != nil {
			SyncErrorsCounter.Add(ctx, 1)
			return nil, err
		}
		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}

		// Unmarshal JSON bytes directly to float32 slice
		if len(vectorBytes) > 0 {
			var vec []float32
			if err := json.Unmarshal(vectorBytes, &vec); err == nil {
				rec.Vector = vec
			}
		}

		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *defaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Prepare update query dynamically for IN clause
	query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN ("
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		if i > 0 {
			query += ", "
		}
		query += fmt.Sprintf("$%d", i+1)
		args[i] = id
	}
	query += ")"

	res, err := s.db.ExecContext(ctx, query, args...)
	if err != nil {
		SyncErrorsCounter.Add(ctx, 1)
		return err
	}

	rowsAffected, _ := res.RowsAffected()
	RecordsSyncedCounter.Add(ctx, rowsAffected)
	return nil
}

func (s *defaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		SyncErrorsCounter.Add(ctx, 1)
		return err
	}
	defer tx.Rollback()

	// Simplified upsert logic, assuming Postges ON CONFLICT or fallback
	for _, rec := range records {

		vectorBytes, err := json.Marshal(rec.Vector)
		if err != nil {
			SyncErrorsCounter.Add(ctx, 1)
			return err
		}

		// Vector columns in Postgres pgvector often expect [1,2,3] as a string representation
		// but since schema definition in 005_sip.sql says `vector_embedding BYTEA` we can store it as json string bytes
		vectorStr := string(vectorBytes)

		// Attempting update first (Last-Write-Wins on Context)
		res, err := tx.ExecContext(ctx, "UPDATE swarm_memory_embeddings SET context = $1, vector_embedding = $2, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $3", rec.Context, vectorStr, rec.ID)
		if err != nil {
			SyncErrorsCounter.Add(ctx, 1)
			return err
		}

		affected, _ := res.RowsAffected()
		if affected == 0 {
			// Insert if not exists
			_, err = tx.ExecContext(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)", rec.ID, rec.Context, vectorStr)
			if err != nil {
				SyncErrorsCounter.Add(ctx, 1)
				return err
			}
		}
		RecordsSyncedCounter.Add(ctx, 1)
	}

	return tx.Commit()
}
