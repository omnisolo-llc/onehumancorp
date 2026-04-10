package hub

import (
	"context"
	"database/sql"
	"time"
	"encoding/json"

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

type dbRAGSyncService struct {
	provider db.Provider
}

func NewDBRAGSyncService(provider db.Provider) RAGSyncService {
	return &dbRAGSyncService{provider: provider}
}

func (s *dbRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		var vectorStr sql.NullString
		if err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSync); err != nil {
			return nil, err
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		if vectorStr.Valid && vectorStr.String != "" {
			// Parse vector from string format [1.0, 2.0] into float slice
			var vec []float32
			if err := json.Unmarshal([]byte(vectorStr.String), &vec); err == nil {
				r.Vector = vec
			}
		}
		records = append(records, r)
	}
	return records, nil
}

func (s *dbRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	var syncedCount int64
	for _, id := range ids {
		_, err := s.provider.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1", id)
		if err != nil {
			telemetry.RecordRagSyncError(ctx, "db_update_error")
			return err
		}
		syncedCount++
	}
	telemetry.RecordRagRecordSynced(ctx, syncedCount)
	return nil
}

func (s *dbRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	var syncedCount int64
	for _, r := range records {
		var vectorStr interface{}
		if len(r.Vector) > 0 {
			b, _ := json.Marshal(r.Vector)
			vectorStr = string(b)
		}

		_, err := s.provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = EXCLUDED.sync_status, last_sync_at = EXCLUDED.last_sync_at", r.ID, r.Context, vectorStr, r.SyncStatus, r.LastSyncAt)
		if err != nil {
			telemetry.RecordRagSyncError(ctx, "db_insert_conflict_error")
			return err
		}
		syncedCount++
	}
	telemetry.RecordRagRecordSynced(ctx, syncedCount)
	return nil
}
