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

type ConcreteRAGSyncService struct {
	dbProvider      db.Provider
	recordsSynced   metric.Int64Counter
	syncErrors      metric.Int64Counter
}

func NewRAGSyncService(dbProvider db.Provider) (*ConcreteRAGSyncService, error) {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSynced, err := meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		return nil, err
	}
	syncErrors, err := meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors during RAG sync"))
	if err != nil {
		return nil, err
	}

	return &ConcreteRAGSyncService{
		dbProvider:    dbProvider,
		recordsSynced: recordsSynced,
		syncErrors:    syncErrors,
	}, nil
}

func (s *ConcreteRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAtStr *string
		var lastSyncAtTime *time.Time
		var vectorStr *string

		if s.dbProvider.IsSQLite() {
			// SQLite driver might return string for dates if not parsed correctly, use string scan
			err = rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSyncAtStr)
			if err == nil && lastSyncAtStr != nil {
				parsed, parseErr := time.Parse(time.RFC3339, *lastSyncAtStr)
				if parseErr == nil {
					r.LastSyncAt = parsed
				}
			}
		} else {
			err = rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSyncAtTime)
			if err == nil && lastSyncAtTime != nil {
				r.LastSyncAt = *lastSyncAtTime
			}
		}

		if err != nil {
			return nil, err
		}

		if vectorStr != nil && *vectorStr != "" {
			err = json.Unmarshal([]byte(*vectorStr), &r.Vector)
			if err != nil {
				// Handle parsing error or skip
				// For now, continue to avoid failing entire batch
			}
		}

		records = append(records, r)
	}

	return records, nil
}

func (s *ConcreteRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		s.syncErrors.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = $1
	`

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			s.syncErrors.Add(ctx, 1)
			return err
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		s.syncErrors.Add(ctx, 1)
		return err
	}

	s.recordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ConcreteRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		s.syncErrors.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	var query string
	if s.dbProvider.IsSQLite() {
		query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, CAST($3 AS TEXT), 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
	} else {
		query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3::vector, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
	}

	for _, record := range records {
		vectorBytes, err := json.Marshal(record.Vector)
		if err != nil {
			s.syncErrors.Add(ctx, 1)
			return err
		}
		vectorStr := string(vectorBytes)

		_, err = tx.Exec(ctx, query, record.ID, record.Context, vectorStr)
		if err != nil {
			s.syncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming sync for memory_id %s: %w", record.ID, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		s.syncErrors.Add(ctx, 1)
		return err
	}

	s.recordsSynced.Add(ctx, int64(len(records)))
	return nil
}
