package hub

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	// "go.opentelemetry.io/otel/metric"
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
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSynced, _ = meter.Int64Counter("rag_records_synced_total")
	syncErrors, _    = meter.Int64Counter("rag_sync_errors_total")
)

type ragSyncService struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncService{db: db}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.Query(ctx, `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`, limit)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vecData []byte
		var lastSync interface{} // Use interface to handle both string, []byte and time.Time

		err := rows.Scan(&rec.ID, &rec.Context, &vecData, &rec.SyncStatus, &lastSync)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return nil, err
		}

		if vecData != nil {
			if err := json.Unmarshal(vecData, &rec.Vector); err != nil {
				syncErrors.Add(ctx, 1)
				return nil, err
			}
		}

		if lastSync != nil {
			switch v := lastSync.(type) {
			case string:
				t, err := time.Parse(time.RFC3339, v)
				if err == nil {
					rec.LastSyncAt = t
				}
			case []byte:
				t, err := time.Parse(time.RFC3339, string(v))
				if err == nil {
					rec.LastSyncAt = t
				}
			case time.Time:
				rec.LastSyncAt = v
			}
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		syncErrors.Add(ctx, 1)
		return nil, err
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	nowStr := now.Format(time.RFC3339)

	for _, id := range ids {
		if s.db.IsSQLite() {
			_, err = tx.Exec(ctx, `
				UPDATE swarm_memory_embeddings
				SET sync_status = 'synced', last_sync_at = $1
				WHERE memory_id = $2
			`, nowStr, id)
		} else {
			_, err = tx.Exec(ctx, `
				UPDATE swarm_memory_embeddings
				SET sync_status = 'synced', last_sync_at = $1
				WHERE memory_id = $2
			`, now, id)
		}

		if err != nil {
			syncErrors.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return err
	}

	recordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return err
	}
	defer tx.Rollback(ctx)

	now := time.Now()
	nowStr := now.Format(time.RFC3339)

	for _, rec := range records {
		var vecData []byte
		if len(rec.Vector) > 0 {
			vecData, _ = json.Marshal(rec.Vector)
		}

		if s.db.IsSQLite() {
			_, err = tx.Exec(ctx, `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', $4)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = 'synced',
					last_sync_at = excluded.last_sync_at
			`, rec.ID, rec.Context, vecData, nowStr)
		} else {
			_, err = tx.Exec(ctx, `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, 'synced', $4)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = 'synced',
					last_sync_at = excluded.last_sync_at
			`, rec.ID, rec.Context, vecData, now)
		}

		if err != nil {
			syncErrors.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return err
	}

	recordsSynced.Add(ctx, int64(len(records)))
	return nil
}
