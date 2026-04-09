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

type DB_RAGSyncService struct {
	Provider db.Provider
}

func (s *DB_RAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	q := "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.Provider.Query(ctx, q, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSync *time.Time
		var vectorStr *string

		err := rows.Scan(&rec.ID, &rec.Context, &vectorStr, &rec.SyncStatus, &lastSync)
		if err != nil {
			return nil, err
		}
		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}
		if vectorStr != nil && *vectorStr != "" {
			var vec []float32
			if err := json.Unmarshal([]byte(*vectorStr), &vec); err == nil {
				rec.Vector = vec
			}
		}
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *DB_RAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	for _, id := range ids {
		q := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
		_, err := s.Provider.Exec(ctx, q, id)
		if err != nil {
			return err
		}
		if telemetry.RagRecordsSyncedTotal != nil {
			telemetry.RagRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return nil
}

func (s *DB_RAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		vectorJSON, _ := json.Marshal(r.Vector)
		vectorStr := string(vectorJSON)

		qUpdate := "UPDATE autodream_memories SET content = $1, embedding = $2, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $3"
		res, err := s.Provider.Exec(ctx, qUpdate, r.Context, vectorStr, r.ID)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}

		rowsAffected := res
		if rowsAffected == 0 {
			qInsert := "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)"
			_, err = s.Provider.Exec(ctx, qInsert, r.ID, r.Context, vectorStr)
			if err != nil {
				if telemetry.RagSyncErrorsTotal != nil {
					telemetry.RagSyncErrorsTotal.Add(ctx, 1)
				}
				return err
			}
		}
		if telemetry.RagRecordsSyncedTotal != nil {
			telemetry.RagRecordsSyncedTotal.Add(ctx, 1)
		}
	}
	return nil
}
