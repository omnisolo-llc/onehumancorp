package hub

import (
	"context"
	"database/sql"
	"fmt"
	"time"


)

// SQLRAGSyncService is a concrete implementation of RAGSyncService backed by an SQL database.
type SQLRAGSyncService struct {
	db       *sql.DB
	isSQLite bool
}

// NewSQLRAGSyncService creates a new SQLRAGSyncService.
func NewSQLRAGSyncService(db *sql.DB, isSQLite bool) *SQLRAGSyncService {
	return &SQLRAGSyncService{db: db, isSQLite: isSQLite}
}

func (s *SQLRAGSyncService) placeholder(i int) string {
	if s.isSQLite {
		return "?"
	}
	return fmt.Sprintf("$%d", i)
}

// FetchPendingSyncs retrieves records from the local DB that need syncing
func (s *SQLRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := fmt.Sprintf(`SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT %s`, s.placeholder(1))

	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud
func (s *SQLRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	query := fmt.Sprintf(`UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = %s WHERE memory_id = %s`, s.placeholder(1), s.placeholder(2))

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to prepare stmt: %w", err)
	}
	defer stmt.Close()

	now := time.Now()
	for _, id := range ids {
		if _, err := stmt.ExecContext(ctx, now, id); err != nil {
			return fmt.Errorf("failed to execute update for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
func (s *SQLRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	query := fmt.Sprintf(`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			  VALUES (%s, %s, %s, 'synced', %s)
			  ON CONFLICT (memory_id) DO UPDATE SET
			  context = EXCLUDED.context,
			  vector_embedding = EXCLUDED.vector_embedding,
			  sync_status = 'synced',
			  last_sync_at = EXCLUDED.last_sync_at`, s.placeholder(1), s.placeholder(2), s.placeholder(3), s.placeholder(4))

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to prepare stmt: %w", err)
	}
	defer stmt.Close()

	now := time.Now()
	for _, r := range records {
		if _, err := stmt.ExecContext(ctx, r.ID, r.Context, r.Vector, now); err != nil {
			return fmt.Errorf("failed to execute upsert for id %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	return nil
}
