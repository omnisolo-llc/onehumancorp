package hub

import (
	"context"
	"database/sql"
	"encoding/binary"
	"math"

	"github.com/onehumancorp/mono/srcs/server/db"
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
	LastSyncAt sql.NullString // Handle both nil and SQLite timestamps
}

var (
	ragRecordsSyncedTotal metric.Int64Counter
	ragSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics(meter metric.Meter) error {
	var err error
	ragRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		return err
	}
	ragSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		return err
	}
	return nil
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type DefaultRAGSyncService struct {
	dbProvider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{dbProvider: provider}
}

func encodeVector(v []float32) []byte {
	buf := make([]byte, len(v)*4)
	for i, f := range v {
		binary.LittleEndian.PutUint32(buf[i*4:], math.Float32bits(f))
	}
	return buf
}

func decodeVector(b []byte) []float32 {
	v := make([]float32, len(b)/4)
	for i := 0; i < len(v); i++ {
		v[i] = math.Float32frombits(binary.LittleEndian.Uint32(b[i*4:]))
	}
	return v
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	if s.dbProvider.IsSQLite() {
		query = `
			SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
			FROM swarm_memory_embeddings
			WHERE sync_status = 'pending'
			LIMIT ?
		`
	}

	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vecBytes []byte
		var syncStatus string
		if err := rows.Scan(&rec.ID, &rec.Context, &vecBytes, &syncStatus, &rec.LastSyncAt); err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return nil, err
		}
		rec.Vector = decodeVector(vecBytes)
		rec.SyncStatus = SyncStatus(syncStatus)
		records = append(records, rec)
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

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = $1
	`
	if s.dbProvider.IsSQLite() {
		query = `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE memory_id = ?
		`
	}

	var successCount int64 = 0

	for _, id := range ids {
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
		successCount++
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, successCount)
	}

	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		if ragSyncErrorsTotal != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
		}
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
	`
	if s.dbProvider.IsSQLite() {
		query = `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
	}

	var successCount int64 = 0

	for _, rec := range records {
		vecBytes := encodeVector(rec.Vector)
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vecBytes)
		if err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
		successCount++
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, successCount)
	}

	return nil
}
