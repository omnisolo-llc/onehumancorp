package hub

import (
	"context"
	"database/sql"
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
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSyncedTotal metric.Int64Counter
	RagSyncErrorsTotal    metric.Int64Counter
)

func InitRAGSyncMetrics() error {
	var err error
	RagRecordsSyncedTotal, err = meter.Int64Counter("rag_records_synced_total")
	if err != nil {
		return fmt.Errorf("failed to initialize rag_records_synced_total: %w", err)
	}
	RagSyncErrorsTotal, err = meter.Int64Counter("rag_sync_errors_total")
	if err != nil {
		return fmt.Errorf("failed to initialize rag_sync_errors_total: %w", err)
	}
	return nil
}

// Convert []float32 to byte slice (little endian)
func float32ArrayToBytes(arr []float32) []byte {
	if arr == nil {
		return nil
	}
	bytes := make([]byte, len(arr)*4)
	for i, f := range arr {
		bits := math.Float32bits(f)
		binary.LittleEndian.PutUint32(bytes[i*4:], bits)
	}
	return bytes
}

// Convert byte slice back to []float32 (little endian)
func bytesToFloat32Array(bytes []byte) []float32 {
	if bytes == nil {
		return nil
	}
	count := len(bytes) / 4
	arr := make([]float32, count)
	for i := 0; i < count; i++ {
		bits := binary.LittleEndian.Uint32(bytes[i*4:])
		arr[i] = math.Float32frombits(bits)
	}
	return arr
}

type ragSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{provider: provider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("FetchPendingSyncs failed: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vecBytes []byte
		var lastSync sql.NullTime

		if err := rows.Scan(&r.ID, &r.Context, &vecBytes, &r.SyncStatus, &lastSync); err != nil {
			return nil, fmt.Errorf("FetchPendingSyncs scan failed: %w", err)
		}

		r.Vector = bytesToFloat32Array(vecBytes)
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}

		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("MarkSynced tx failed: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
		if _, err := tx.Exec(ctx, query, id); err != nil {
			if RagSyncErrorsTotal != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("MarkSynced update failed for id %s: %w", id, err)
		}
		if RagRecordsSyncedTotal != nil {
			RagRecordsSyncedTotal.Add(ctx, 1)
		}
	}

	return tx.Commit(ctx)
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("ProcessIncomingSync tx failed: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, r := range records {
		vecBytes := float32ArrayToBytes(r.Vector)
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
		if _, err := tx.Exec(ctx, query, r.ID, r.Context, vecBytes); err != nil {
			if RagSyncErrorsTotal != nil {
				RagSyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("ProcessIncomingSync insert failed for id %s: %w", r.ID, err)
		}
		if RagRecordsSyncedTotal != nil {
			RagRecordsSyncedTotal.Add(ctx, 1)
		}
	}

	return tx.Commit(ctx)
}
