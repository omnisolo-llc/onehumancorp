package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
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

type sqlRAGSyncService struct {
	db db.Provider
}

func NewSQLRAGSyncService(db db.Provider) RAGSyncService {
	return &sqlRAGSyncService{db: db}
}

func (s *sqlRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT "
	if s.db.IsSQLite() {
		query += "?"
	} else {
		query += "$1"
	}

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSync *time.Time
		var vector []byte
		var syncStatus *string

		err := rows.Scan(&rec.ID, &rec.Context, &vector, &syncStatus, &lastSync)
		if err != nil {
			return nil, err
		}
		rec.Vector = vector
		if syncStatus != nil {
			rec.SyncStatus = SyncStatus(*syncStatus)
		} else {
			rec.SyncStatus = SyncStatusPending
		}
		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *sqlRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	now := time.Now()
	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = "
		if s.db.IsSQLite() {
			query += "? WHERE memory_id = ?"
		} else {
			query += "$1 WHERE memory_id = $2"
		}

		_, err := s.db.Exec(ctx, query, now, id)
		if err != nil {
			if telemetry.RAGSyncErrorsTotalCounter != nil {
				telemetry.RAGSyncErrorsTotalCounter.Add(ctx, 1)
			}
			return err
		}
		if telemetry.RAGRecordsSyncedTotalCounter != nil {
			telemetry.RAGRecordsSyncedTotalCounter.Add(ctx, 1)
		}
	}
	return nil
}

func (s *sqlRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	now := time.Now()
	for _, rec := range records {
		var query string
		if s.db.IsSQLite() {
			query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, 'synced', ?)
			ON CONFLICT(memory_id) DO UPDATE SET
			context = excluded.context,
			vector_embedding = excluded.vector_embedding,
			sync_status = 'synced',
			last_sync_at = excluded.last_sync_at`
			_, err := s.db.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, now)
			if err != nil {
				if telemetry.RAGSyncErrorsTotalCounter != nil {
					telemetry.RAGSyncErrorsTotalCounter.Add(ctx, 1)
				}
				return err
			}
		} else {
			query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT(memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = 'synced',
			last_sync_at = EXCLUDED.last_sync_at`
			_, err := s.db.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, now)
			if err != nil {
				if telemetry.RAGSyncErrorsTotalCounter != nil {
					telemetry.RAGSyncErrorsTotalCounter.Add(ctx, 1)
				}
				return err
			}
		}
		if telemetry.RAGRecordsSyncedTotalCounter != nil {
			telemetry.RAGRecordsSyncedTotalCounter.Add(ctx, 1)
		}
	}
	return nil
}
