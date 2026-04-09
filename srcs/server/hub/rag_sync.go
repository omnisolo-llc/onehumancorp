package hub

import (
	"context"
	"encoding/json"
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
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	DB db.Provider
}

func NewRAGSyncService(database db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{DB: database}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.DB.Query(ctx, query, limit)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var status string
		var vecBytes []byte
		err := rows.Scan(&r.ID, &r.Context, &vecBytes, &status, &lastSyncAt)
		if err == nil && len(vecBytes) > 0 {
			_ = json.Unmarshal(vecBytes, &r.Vector)
		}
		if err != nil {
			return nil, err
		}
		r.SyncStatus = SyncStatus(status)
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1"
		_, err := s.DB.Exec(ctx, query, id)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
	}

	telemetry.RecordRAGRecordSynced(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		vecBytes, _ := json.Marshal(r.Vector)
		query := "UPDATE swarm_memory_embeddings SET context = $1, vector_embedding = $2, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $3"
		affected, err := s.DB.Exec(ctx, query, r.Context, vecBytes, r.ID)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}

		if affected == 0 {
			query = "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)"
			_, err = s.DB.Exec(ctx, query, r.ID, r.Context, vecBytes)
			if err != nil {
				telemetry.RecordRAGSyncError(ctx)
				return err
			}
		}
	}

	telemetry.RecordRAGRecordSynced(ctx, int64(len(records)))
	return nil
}
