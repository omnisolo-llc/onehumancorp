package hub

import (
	"context"
	"time"
	"unsafe"
	"fmt"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
	"github.com/onehumancorp/mono/srcs/server/db"
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

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt *time.Time
		var syncStatus *string
		var vectorBytes []byte
		if err := rows.Scan(&r.ID, &r.Context, &vectorBytes, &syncStatus, &lastSyncAt); err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return nil, err
		}
		if syncStatus != nil {
			r.SyncStatus = SyncStatus(*syncStatus)
		} else {
			r.SyncStatus = SyncStatusPending
		}
		if lastSyncAt != nil {
			r.LastSyncAt = *lastSyncAt
		}
		// Convert raw bytes to float32 slice (simplified for SQLite/Postgres hybrid)
		// Assuming vectorBytes is correctly formatted byte array
		if len(vectorBytes) > 0 {
			r.Vector = make([]float32, len(vectorBytes)/4)
			for i := range r.Vector {
				// Very basic byte-to-float32 conversion
				var v uint32 = uint32(vectorBytes[i*4]) | uint32(vectorBytes[i*4+1])<<8 | uint32(vectorBytes[i*4+2])<<16 | uint32(vectorBytes[i*4+3])<<24
				r.Vector[i] = *(*float32)(unsafe.Pointer(&v))
			}
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2"
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			return fmt.Errorf("failed to update record %s: %w", id, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}
	RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		var vectorBytes []byte
		if len(rec.Vector) > 0 {
			vectorBytes = make([]byte, len(rec.Vector)*4)
			for i, v := range rec.Vector {
				vu := *(*uint32)(unsafe.Pointer(&v))
				vectorBytes[i*4] = byte(vu)
				vectorBytes[i*4+1] = byte(vu >> 8)
				vectorBytes[i*4+2] = byte(vu >> 16)
				vectorBytes[i*4+3] = byte(vu >> 24)
			}
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorBytes, string(rec.SyncStatus), rec.LastSyncAt)
		if err != nil {
			SyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to process incoming record %s: %w", rec.ID, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		SyncErrorsTotal.Add(ctx, 1)
		return err
	}
	RecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}

var (
	meter                   = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RecordsSyncedTotal, _   = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	SyncErrorsTotal, _      = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of errors during RAG sync"))
)
