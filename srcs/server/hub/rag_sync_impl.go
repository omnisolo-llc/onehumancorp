package hub

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type dbRAGSyncService struct {
	db db.Provider
}

// NewDBRAGSyncService creates a new RAGSyncService backed by the database.
func NewDBRAGSyncService(db db.Provider) RAGSyncService {
	return &dbRAGSyncService{db: db}
}

func (s *dbRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
			  FROM swarm_memory_embeddings
			  WHERE sync_status = 'pending' LIMIT `

	if s.db.IsSQLite() {
		query += fmt.Sprintf("%d", limit)
	} else {
		query += fmt.Sprintf("$1")
	}

	var rows db.Rows
	var err error
	if s.db.IsSQLite() {
		rows, err = s.db.Query(ctx, query)
	} else {
		rows, err = s.db.Query(ctx, query, limit)
	}

	if err != nil {
		telemetry.RecordRAGSyncError(ctx, "FetchPendingSyncs query failed")
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSync); err != nil {
			telemetry.RecordRAGSyncError(ctx, "FetchPendingSyncs scan failed")
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

func (s *dbRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	now := time.Now()
	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = `
		if s.db.IsSQLite() {
			query += `? WHERE memory_id = ?`
			_, err := s.db.Exec(ctx, query, now, id)
			if err != nil {
				telemetry.RecordRAGSyncError(ctx, "MarkSynced exec failed")
				return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
			}
		} else {
			query += `$1 WHERE memory_id = $2`
			_, err := s.db.Exec(ctx, query, now, id)
			if err != nil {
				telemetry.RecordRAGSyncError(ctx, "MarkSynced exec failed")
				return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
			}
		}
	}

	telemetry.RecordRAGRecordsSynced(ctx, int64(len(ids)))
	return nil
}

func (s *dbRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	now := time.Now()
	for _, r := range records {
		var query string
		if s.db.IsSQLite() {
			query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
					 VALUES (?, ?, ?, 'synced', ?)
					 ON CONFLICT (memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = 'synced', last_sync_at = excluded.last_sync_at`
			_, err := s.db.Exec(ctx, query, r.ID, r.Context, r.Vector, now)
			if err != nil {
				telemetry.RecordRAGSyncError(ctx, "ProcessIncomingSync exec failed")
				return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
			}
		} else {
			query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
					 VALUES ($1, $2, $3, 'synced', $4)
					 ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = EXCLUDED.last_sync_at`
			_, err := s.db.Exec(ctx, query, r.ID, r.Context, r.Vector, now)
			if err != nil {
				telemetry.RecordRAGSyncError(ctx, "ProcessIncomingSync exec failed")
				return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
			}
		}
	}

	telemetry.RecordRAGRecordsSynced(ctx, int64(len(records)))
	return nil
}
