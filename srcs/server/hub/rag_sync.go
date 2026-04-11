package hub

import (
	"context"
	"database/sql"
	"fmt"
	"strings"
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

type ragSyncServiceImpl struct {
	db db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{db: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"

	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSync); err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, err
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		records = append(records, r)
	}

	return records, rows.Err()
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids)+2)
	for i, id := range ids {
		if s.db.IsSQLite() {
			placeholders[i] = "?"
		} else {
			placeholders[i] = fmt.Sprintf("$%d", i+3)
		}
		args[i+2] = id
	}

	args[0] = string(SyncStatusSynced)
	args[1] = time.Now()

	var p1, p2 string
	if s.db.IsSQLite() {
		p1 = "?"
		p2 = "?"
	} else {
		p1 = "$1"
		p2 = "$2"
	}

	query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = %s, last_sync_at = %s WHERE memory_id IN (%s)",
		p1, p2, strings.Join(placeholders, ","))

	_, err = tx.Exec(ctx, query, args...)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	if err := tx.Commit(ctx); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		if s.db.IsSQLite() {
			query := "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(memory_id) DO UPDATE SET context=excluded.context, vector_embedding=excluded.vector_embedding, sync_status=excluded.sync_status, last_sync_at=excluded.last_sync_at"
			_, err = tx.Exec(ctx, query, r.ID, r.Context, r.Vector, string(r.SyncStatus), r.LastSyncAt)
		} else {
			query := "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, $4, $5) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, vector_embedding=EXCLUDED.vector_embedding, sync_status=EXCLUDED.sync_status, last_sync_at=EXCLUDED.last_sync_at"
			_, err = tx.Exec(ctx, query, r.ID, r.Context, r.Vector, string(r.SyncStatus), r.LastSyncAt)
		}

		if err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}

	return nil
}
