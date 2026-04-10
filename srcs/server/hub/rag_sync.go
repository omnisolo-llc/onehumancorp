package hub

import (
	"context"
	"encoding/json"
	"fmt"
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
	Vector     []float32 // Convert to string internally for SQLite compat if needed
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type RAGSyncServiceImpl struct {
	db *db.DB
}

func NewRAGSyncService(db *db.DB) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{db: db}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if !s.db.IsSQLite() {
		return nil, fmt.Errorf("FetchPendingSyncs only supported on SQLite")
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	rows, err := tx.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vecStr *string
		var lastSync *time.Time
		if err := rows.Scan(&r.ID, &r.Context, &vecStr, &r.SyncStatus, &lastSync); err != nil {
			return nil, err
		}
		if vecStr != nil {
			if err := json.Unmarshal([]byte(*vecStr), &r.Vector); err != nil {
				return nil, err
			}
		}
		if lastSync != nil {
			r.LastSyncAt = *lastSync
		}
		records = append(records, r)
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	if !s.db.IsSQLite() {
		return fmt.Errorf("MarkSynced only supported on SQLite")
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC()
	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2"
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		var vecParam interface{}
		if s.db.IsSQLite() {
			vecBytes, err := json.Marshal(r.Vector)
			if err != nil {
				if telemetry.RagSyncErrorsTotal != nil { telemetry.RagSyncErrorsTotal.Add(ctx, 1) }
				return err
			}
			vecParam = string(vecBytes)
		} else {
			vecParam = r.Vector
		}

		query := `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (id) DO UPDATE SET
			content = excluded.content,
			embedding = excluded.embedding,
			sync_status = excluded.sync_status,
			last_sync_at = excluded.last_sync_at
		`
		_, err := tx.Exec(ctx, query, r.ID, r.Context, vecParam, r.SyncStatus, r.LastSyncAt)
		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil { telemetry.RagSyncErrorsTotal.Add(ctx, 1) }
			return err
		}
		if telemetry.RagRecordsSyncedTotal != nil { telemetry.RagRecordsSyncedTotal.Add(ctx, 1) }
	}

	return tx.Commit(ctx)
}
