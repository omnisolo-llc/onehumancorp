package hub

import (
	"context"
	"encoding/binary"
	"fmt"
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
	Vector     []float32
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

type ragSyncProvider struct {
	provider    db.Provider
	syncTotal   metric.Int64Counter
	errorsTotal metric.Int64Counter
}

// Global metrics initialized once
var (
	meter          = otel.Meter("hybrid_rag_sync")
	ragSyncTotal, _   = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total RAG records synced"))
	ragErrorsTotal, _ = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total RAG sync errors"))
)

func NewRAGSyncProvider(provider db.Provider) RAGSyncService {
	return &ragSyncProvider{
		provider:    provider,
		syncTotal:   ragSyncTotal,
		errorsTotal: ragErrorsTotal,
	}
}

func (s *ragSyncProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending' OR sync_status IS NULL
		LIMIT $1
	`
	if s.provider.IsSQLite() {
		query = `
			SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
			FROM swarm_memory_embeddings
			WHERE sync_status = 'pending' OR sync_status IS NULL
			LIMIT ?
		`
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vecBytes []byte
		var syncStatus string
		var lastSyncAt *time.Time

		err := rows.Scan(&rec.ID, &rec.Context, &vecBytes, &syncStatus, &lastSyncAt)
		if err != nil {
			s.errorsTotal.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}

		rec.SyncStatus = SyncStatus(syncStatus)
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}

		if len(vecBytes) > 0 {
			vec := make([]float32, len(vecBytes)/4)
			for i := 0; i < len(vec); i++ {
				bits := binary.LittleEndian.Uint32(vecBytes[i*4 : (i+1)*4])
				vec[i] = math.Float32frombits(bits)
			}
			rec.Vector = vec
		}

		records = append(records, rec)
	}

	return records, nil
}

func (s *ragSyncProvider) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
	if s.provider.IsSQLite() {
		query = `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = ?`
	}

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			s.errorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	s.syncTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = CURRENT_TIMESTAMP
	`
	if s.provider.IsSQLite() {
		query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = excluded.sync_status,
				last_sync_at = CURRENT_TIMESTAMP
		`
	}

	for _, rec := range records {
		var vecBytes []byte
		if len(rec.Vector) > 0 {
			vecBytes = make([]byte, len(rec.Vector)*4)
			for i, v := range rec.Vector {
				binary.LittleEndian.PutUint32(vecBytes[i*4:(i+1)*4], math.Float32bits(v))
			}
		}

		status := "synced" // Since it's processed in cloud, mark as synced
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vecBytes, status)
		if err != nil {
			s.errorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record id %s: %w", rec.ID, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		s.errorsTotal.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	s.syncTotal.Add(ctx, int64(len(records)))
	return nil
}
