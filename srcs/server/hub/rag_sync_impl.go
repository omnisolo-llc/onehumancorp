package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
)

type Rows interface {
	Next() bool
	Scan(dest ...any) error
	Close() error
	Err() error
	Columns() ([]string, error)
}

type Tx interface {
	ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error)
	Commit() error
	Rollback() error
}

type DBProvider interface {
	QueryContext(ctx context.Context, query string, args ...any) (Rows, error)
	ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error)
	BeginTx(ctx context.Context, opts *sql.TxOptions) (Tx, error)
}

type ragSyncService struct {
	db DBProvider
}

func NewRAGSyncService(db DBProvider) RAGSyncService {
	return &ragSyncService{db: db}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
              FROM swarm_memory_embeddings
              WHERE sync_status = 'pending'
              LIMIT $1`

	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorJSON []byte
		var lastSyncAt sql.NullTime

		if err := rows.Scan(&r.ID, &r.Context, &vectorJSON, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		if vectorJSON != nil {
			if err := json.Unmarshal(vectorJSON, &r.Vector); err != nil {
				// Ignore error to keep processing other rows
			}
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}

		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}
	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
	var successCount int64
	for _, id := range ids {
		if _, err := tx.ExecContext(ctx, query, id); err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
		successCount++
	}

	if err := tx.Commit(); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	RagRecordsSyncedTotal.Add(ctx, successCount)
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `
        INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
        VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
        ON CONFLICT(memory_id) DO UPDATE SET
            context = excluded.context,
            vector_embedding = excluded.vector_embedding,
            sync_status = excluded.sync_status,
            last_sync_at = excluded.last_sync_at
    `

	for _, r := range records {
		vectorJSON, _ := json.Marshal(r.Vector)
		if _, err := tx.ExecContext(ctx, query, r.ID, r.Context, string(vectorJSON), r.SyncStatus); err != nil {
			return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
		}
	}

	return tx.Commit()
}
