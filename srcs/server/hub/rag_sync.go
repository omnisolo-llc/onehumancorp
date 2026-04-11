package hub

import (
	"context"
	"database/sql"
	"time"
	"github.com/onehumancorp/mono/srcs/server/db"
	"strings"
	"strconv"
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

type DBRAGSyncService struct {
	dbProvider db.Provider
}

func NewDBRAGSyncService(provider db.Provider) RAGSyncService {
	return &DBRAGSyncService{dbProvider: provider}
}

func (s *DBRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSync); err != nil {
			return nil, err
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *DBRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// Convert ids to interface slice
	args := make([]interface{}, len(ids))
	placeholders := make([]string, len(ids))
	for i, id := range ids {
		args[i] = id
		placeholders[i] = "?"
	}

	query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (" + strings.Join(placeholders, ",") + ")"
    if !s.dbProvider.IsSQLite() {
        for i := range placeholders {
			placeholders[i] = "$" + strconv.Itoa(i+1)
		}
        query = "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (" + strings.Join(placeholders, ",") + ")"
    }

	_, err := s.dbProvider.Exec(ctx, query, args...)
	return err
}

func (s *DBRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		var query string
		if s.dbProvider.IsSQLite() {
			query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
			_, err := s.dbProvider.Exec(ctx, query, r.ID, r.Context, r.Vector)
			if err != nil {
				return err
			}
		} else {
			query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT(memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP`
			_, err := s.dbProvider.Exec(ctx, query, r.ID, r.Context, r.Vector)
			if err != nil {
				return err
			}
		}
	}
	return nil
}
