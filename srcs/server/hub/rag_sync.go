package hub

import (
	"context"
	"database/sql"
	"encoding/binary"
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
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

	RecordsSyncedTotal, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to the cloud"),
	)
	SyncErrorsTotal, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
)

type SyncDaemon struct {
	provider db.Provider
}

func NewSyncDaemon(provider db.Provider) *SyncDaemon {
	return &SyncDaemon{provider: provider}
}

func encodeVector(vec []float32) []byte {
	if vec == nil {
		return nil
	}
	buf := make([]byte, len(vec)*4)
	for i, f := range vec {
		binary.LittleEndian.PutUint32(buf[i*4:], math.Float32bits(f))
	}
	return buf
}

func decodeVector(buf []byte) []float32 {
	if buf == nil {
		return nil
	}
	if len(buf)%4 != 0 {
		return nil
	}
	vec := make([]float32, len(buf)/4)
	for i := range vec {
		vec[i] = math.Float32frombits(binary.LittleEndian.Uint32(buf[i*4 : (i+1)*4]))
	}
	return vec
}

func (s *SyncDaemon) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		var vectorBytes []byte

		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		rec.Vector = decodeVector(vectorBytes)

		records = append(records, rec)
	}

	return records, rows.Err()
}

func (s *SyncDaemon) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE memory_id = $1
		`
		if _, err := s.provider.Exec(ctx, query, id); err != nil {
			return err
		}
	}
	return nil
}

func (s *SyncDaemon) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
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
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE
			SET context = EXCLUDED.context,
			    vector_embedding = EXCLUDED.vector_embedding,
			    sync_status = 'synced',
			    last_sync_at = CURRENT_TIMESTAMP
		`
		vectorBytes := encodeVector(rec.Vector)

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorBytes)
		if err != nil {
			return err
		}

		RecordsSyncedTotal.Add(ctx, 1)
	}

	return tx.Commit(ctx)
}
