package hub

import (
	"context"
	"encoding/binary"
	"fmt"
	"math"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric"
)

var (
	RecordsSyncedTotal metric.Int64Counter
	SyncErrorsTotal    metric.Int64Counter
)

// InitRAGSyncMetrics initializes OpenTelemetry metrics for the Hybrid MCP RAG Protocol.
func InitRAGSyncMetrics(meter metric.Meter) error {
	var err error
	RecordsSyncedTotal, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records synced to the cloud"),
	)
	if err != nil {
		return err
	}

	SyncErrorsTotal, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors encountered during RAG sync"),
	)
	if err != nil {
		return err
	}
	return nil
}

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

type DefaultRAGSyncService struct {
	db db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{db: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending' OR sync_status IS NULL
		LIMIT $1
	`, limit)
	if err != nil {
		if SyncErrorsTotal != nil {
			SyncErrorsTotal.Add(ctx, 1)
		}
		return nil, fmt.Errorf("query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorBytes []byte
		var syncStatus *string
		var lastSyncAt *time.Time

		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &syncStatus, &lastSyncAt); err != nil {
			if SyncErrorsTotal != nil {
				SyncErrorsTotal.Add(ctx, 1)
			}
			return nil, fmt.Errorf("scan pending sync: %w", err)
		}

		if syncStatus != nil {
			rec.SyncStatus = SyncStatus(*syncStatus)
		} else {
			rec.SyncStatus = SyncStatusPending
		}

		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}

		if len(vectorBytes) > 0 {
			rec.Vector = make([]float32, len(vectorBytes)/4)
			for i := 0; i < len(rec.Vector); i++ {
				bits := binary.LittleEndian.Uint32(vectorBytes[i*4 : (i+1)*4])
				rec.Vector[i] = math.Float32frombits(bits)
			}
		}

		records = append(records, rec)
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		if SyncErrorsTotal != nil {
			SyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		_, err := tx.Exec(ctx, `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE memory_id = $1
		`, id)
		if err != nil {
			if SyncErrorsTotal != nil {
				SyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("update sync status for %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if SyncErrorsTotal != nil {
			SyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("commit tx: %w", err)
	}

	if RecordsSyncedTotal != nil {
		RecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		if SyncErrorsTotal != nil {
			SyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		vectorBytes := make([]byte, len(rec.Vector)*4)
		for i, v := range rec.Vector {
			bits := math.Float32bits(v)
			binary.LittleEndian.PutUint32(vectorBytes[i*4:(i+1)*4], bits)
		}

		if s.db.IsSQLite() {
			_, err = tx.Exec(ctx, `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`, rec.ID, rec.Context, vectorBytes)
		} else {
			_, err = tx.Exec(ctx, `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`, rec.ID, rec.Context, vectorBytes)
		}

		if err != nil {
			if SyncErrorsTotal != nil {
				SyncErrorsTotal.Add(ctx, 1)
			}
			return fmt.Errorf("upsert incoming record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if SyncErrorsTotal != nil {
			SyncErrorsTotal.Add(ctx, 1)
		}
		return fmt.Errorf("commit tx: %w", err)
	}

	if RecordsSyncedTotal != nil {
		RecordsSyncedTotal.Add(ctx, int64(len(records)))
	}

	return nil
}
