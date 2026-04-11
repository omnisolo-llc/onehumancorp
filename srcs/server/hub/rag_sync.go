package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Content    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt *time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type HybridRAGSyncService struct {
	dbProvider         db.Provider
	recordsSyncedTotal metric.Int64Counter
	syncErrorsTotal    metric.Int64Counter
}

func NewHybridRAGSyncService(dbProvider db.Provider) *HybridRAGSyncService {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	syncedTotal, _ := meter.Int64Counter("rag_records_synced_total")
	errorsTotal, _ := meter.Int64Counter("rag_sync_errors_total")
	return &HybridRAGSyncService{
		dbProvider:         dbProvider,
		recordsSyncedTotal: syncedTotal,
		syncErrorsTotal:    errorsTotal,
	}
}

func (s *HybridRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.dbProvider.Query(ctx, "SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		if s.syncErrorsTotal != nil {
			s.syncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var record RAGSyncRecord
		var vectorStr *string
		if err := rows.Scan(&record.ID, &record.Content, &vectorStr, &record.SyncStatus, &record.LastSyncAt); err != nil {
			if s.syncErrorsTotal != nil {
				s.syncErrorsTotal.Add(ctx, 1)
			}
			return nil, err
		}
		if vectorStr != nil {
			var vector []float32
			if err := json.Unmarshal([]byte(*vectorStr), &vector); err == nil {
				record.Vector = vector
			}
		}
		records = append(records, record)
	}
	return records, nil
}

func (s *HybridRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		if s.syncErrorsTotal != nil {
			s.syncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1", id)
		if err != nil {
			if s.syncErrorsTotal != nil {
				s.syncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if s.syncErrorsTotal != nil {
			s.syncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	if s.recordsSyncedTotal != nil {
		s.recordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (s *HybridRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		if s.syncErrorsTotal != nil {
			s.syncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	for _, record := range records {
		var vectorParam interface{} = nil
		if len(record.Vector) > 0 {
			b, _ := json.Marshal(record.Vector)
			vectorParam = string(b)
		}

		query := ""
		if s.dbProvider.IsSQLite() {
			query = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CAST($3 AS TEXT), $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at`
		} else {
			query = `
			INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3::vector, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at`
		}

		_, err := tx.Exec(ctx, query, record.ID, record.Content, vectorParam, record.SyncStatus, record.LastSyncAt)
		if err != nil {
			if s.syncErrorsTotal != nil {
				s.syncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("failed to process incoming sync: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if s.syncErrorsTotal != nil {
			s.syncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	if s.recordsSyncedTotal != nil {
		s.recordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}
