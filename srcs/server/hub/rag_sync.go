package hub

import (
	"context"
	"encoding/binary"
	"fmt"
	"log/slog"
	"math"
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

var (
	meter                = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func init() {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		slog.Error("failed to create rag_records_synced_total metric", "error", err)
	}

	ragSyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
	if err != nil {
		slog.Error("failed to create rag_sync_errors_total metric", "error", err)
	}
}

type ragSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		provider: provider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var id string
		var ctxStr string
		var vecBytes []byte
		var status string
		var lastSync *time.Time

		if err := rows.Scan(&id, &ctxStr, &vecBytes, &status, &lastSync); err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}

		var vector []float32
		if len(vecBytes) > 0 {
			if len(vecBytes)%4 != 0 {
				ragSyncErrorsTotal.Add(ctx, 1)
				slog.Error("invalid vector_embedding length", "id", id, "length", len(vecBytes))
			} else {
				vector = make([]float32, len(vecBytes)/4)
				for i := 0; i < len(vector); i++ {
					bits := binary.LittleEndian.Uint32(vecBytes[i*4 : (i+1)*4])
					vector[i] = math.Float32frombits(bits)
				}
			}
		}

		rec := RAGSyncRecord{
			ID:         id,
			Context:    ctxStr,
			Vector:     vector,
			SyncStatus: SyncStatus(status),
		}
		if lastSync != nil {
			rec.LastSyncAt = *lastSync
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// This is a naive implementation; ideally use a transaction and batched queries or IN clause
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	for _, id := range ids {
		_, err := tx.Exec(ctx, `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2`, now, id)
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update sync_status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var vecBytes []byte
		if len(rec.Vector) > 0 {
			vecBytes = make([]byte, len(rec.Vector)*4)
			for i, v := range rec.Vector {
				bits := math.Float32bits(v)
				binary.LittleEndian.PutUint32(vecBytes[i*4:(i+1)*4], bits)
			}
		}

		// Use ON CONFLICT DO UPDATE
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', $4)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_at = excluded.last_sync_at
		`
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vecBytes, time.Now())
		if err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
