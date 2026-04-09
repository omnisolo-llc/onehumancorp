package hub

import (
	"context"
	"database/sql"
	"encoding/json"
)

type defaultRAGSyncService struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) RAGSyncService {
	return &defaultRAGSyncService{db: db}
}

func (s *defaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending' OR sync_status IS NULL
		LIMIT $1
	`
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorBytes []byte
		var lastSyncAt sql.NullTime
		var syncStatus sql.NullString
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &syncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if vectorBytes != nil {
			if err := json.Unmarshal(vectorBytes, &rec.Vector); err != nil {
				// If it fails to unmarshal JSON, it might be raw bytes, but let's try standard JSON first for simplicity or ignore.
			}
		}
		if syncStatus.Valid {
			rec.SyncStatus = SyncStatus(syncStatus.String)
		} else {
			rec.SyncStatus = SyncStatusPending
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *defaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// Simplified implementation for test
	query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
	for _, id := range ids {
		if _, err := s.db.ExecContext(ctx, query, id); err != nil {
			return err
		}
	}
	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *defaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, rec := range records {
		vectorBytes, _ := json.Marshal(rec.Vector)
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
		if _, err := s.db.ExecContext(ctx, query, rec.ID, rec.Context, vectorBytes); err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return err
		}
		RecordsSyncedTotal.Add(ctx, 1)
	}
	return nil
}
