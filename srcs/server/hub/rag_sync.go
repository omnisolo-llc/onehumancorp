package hub

import (
	"context"
	"database/sql"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SyncStatus string

const (
	SyncStatusPending    SyncStatus = "pending"
	SyncStatusInProgress SyncStatus = "in_progress"
	SyncStatusSynced     SyncStatus = "synced"
	SyncStatusError      SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []byte
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var rows db.Rows
	var err error

	if s.provider.IsSQLite() {
		// SQLite: simply query, it's single user so no FOR UPDATE SKIP LOCKED
		rows, err = s.provider.Query(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = ? LIMIT ?", string(SyncStatusPending), limit)
	} else {
		// Postgres: update to in_progress returning to prevent race conditions
		updateQuery := `
			UPDATE swarm_memory_embeddings
			SET sync_status = $1
			WHERE memory_id IN (
				SELECT memory_id
				FROM swarm_memory_embeddings
				WHERE sync_status = $2
				LIMIT $3
				FOR UPDATE SKIP LOCKED
			)
			RETURNING memory_id, context, vector_embedding, sync_status, last_sync_at
		`
		rows, err = s.provider.Query(ctx, updateQuery, string(SyncStatusInProgress), string(SyncStatusPending), limit)
	}

	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var syncStatusStr string
		var lastSyncAt sql.NullTime

		if err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &syncStatusStr, &lastSyncAt); err != nil {
			telemetry.RagSyncErrorsCounter.Add(ctx, 1)
			return nil, err
		}

		rec.SyncStatus = SyncStatus(syncStatusStr)
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}

	// SQLite specific race prevention logic: if SQLite, we didn't update status in query.
	// Since SQLite is single-user, this is less critical, but we can do it here for consistency if needed.
	if s.provider.IsSQLite() && len(records) > 0 {
		for _, rec := range records {
			_, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = ? WHERE memory_id = ?", string(SyncStatusInProgress), rec.ID)
			if err != nil {
				telemetry.RagSyncErrorsCounter.Add(ctx, 1)
				return nil, err
			}
		}
	}

	return records, rows.Err()
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	now := time.Now().UTC()
	for _, id := range ids {
		if s.provider.IsSQLite() {
			_, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = ?, last_sync_at = ? WHERE memory_id = ?", string(SyncStatusSynced), now, id)
			if err != nil {
				telemetry.RagSyncErrorsCounter.Add(ctx, 1)
				return err
			}
		} else {
			_, err := s.provider.Exec(ctx, "UPDATE swarm_memory_embeddings SET sync_status = $1, last_sync_at = $2 WHERE memory_id = $3", string(SyncStatusSynced), now, id)
			if err != nil {
				telemetry.RagSyncErrorsCounter.Add(ctx, 1)
				return err
			}
		}
		telemetry.RagRecordsSyncedCounter.Add(ctx, 1)
	}
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, rec := range records {
		if s.provider.IsSQLite() {
			_, err := s.provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(memory_id) DO UPDATE SET context=excluded.context, vector_embedding=excluded.vector_embedding, sync_status=excluded.sync_status, last_sync_at=excluded.last_sync_at", rec.ID, rec.Context, rec.Vector, string(SyncStatusSynced), rec.LastSyncAt)
			if err != nil {
				telemetry.RagSyncErrorsCounter.Add(ctx, 1)
				return err
			}
		} else {
			_, err := s.provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, vector_embedding=EXCLUDED.vector_embedding, sync_status=EXCLUDED.sync_status, last_sync_at=EXCLUDED.last_sync_at", rec.ID, rec.Context, rec.Vector, string(SyncStatusSynced), rec.LastSyncAt)
			if err != nil {
				telemetry.RagSyncErrorsCounter.Add(ctx, 1)
				return err
			}
		}
		telemetry.RagRecordsSyncedCounter.Add(ctx, 1)
	}
	return nil
}
