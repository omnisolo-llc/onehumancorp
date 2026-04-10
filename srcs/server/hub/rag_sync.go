package hub

import (
	"context"
	"database/sql"
	"encoding/binary"
	"math"
	"time"

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

var (
	meter                = otel.Meter("hub_rag_sync")
	RagRecordsSyncedTotal, _ = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	RagSyncErrorsTotal, _ = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of RAG sync errors"),
	)
)

type RAGSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{provider: provider}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT "
	if s.provider.IsSQLite() {
		query += "?"
	} else {
		query += "$1"
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vectorBytes []byte
		var lastSync sql.NullString
		err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &rec.SyncStatus, &lastSync)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return nil, err
		}

		if len(vectorBytes) > 0 {
			rec.Vector = make([]float32, len(vectorBytes)/4)
			for i := 0; i < len(rec.Vector); i++ {
				bits := binary.LittleEndian.Uint32(vectorBytes[i*4 : (i+1)*4])
				rec.Vector[i] = math.Float32frombits(bits)
			}
		}

		if lastSync.Valid {
			if t, err := time.Parse(time.RFC3339, lastSync.String); err == nil {
				rec.LastSyncAt = t
			} else if t, err := time.Parse("2006-01-02 15:04:05-07:00", lastSync.String); err == nil {
				rec.LastSyncAt = t
			} else if t, err := time.Parse("2006-01-02 15:04:05", lastSync.String); err == nil {
				rec.LastSyncAt = t
			}
		}

		records = append(records, rec)
	}

	return records, rows.Err()
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC().Format(time.RFC3339)

	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = "
		if s.provider.IsSQLite() {
			query += "? WHERE memory_id = ?"
		} else {
			query += "$1 WHERE memory_id = $2"
		}
		_, err := tx.Exec(ctx, query, now, id)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC().Format(time.RFC3339)

	for _, rec := range records {
		var vectorBytes []byte
		if len(rec.Vector) > 0 {
			vectorBytes = make([]byte, len(rec.Vector)*4)
			for i, v := range rec.Vector {
				binary.LittleEndian.PutUint32(vectorBytes[i*4:(i+1)*4], math.Float32bits(v))
			}
		}

		query := "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ("
		if s.provider.IsSQLite() {
			query += "?, ?, ?, 'synced', ?) ON CONFLICT (memory_id) DO UPDATE SET context = excluded.context, vector_embedding = excluded.vector_embedding, sync_status = 'synced', last_sync_at = excluded.last_sync_at"
		} else {
			query += "$1, $2, $3, 'synced', $4) ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced', last_sync_at = EXCLUDED.last_sync_at"
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorBytes, now)
		if err != nil {
			RagSyncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		RagSyncErrorsTotal.Add(ctx, 1)
		return err
	}

	RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
