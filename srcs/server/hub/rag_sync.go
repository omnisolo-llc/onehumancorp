package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
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
	recordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	syncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total")
)

type ragSyncService struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncService{dbProvider: dbProvider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
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
		var rec RAGSyncRecord
		var lastSync sql.NullTime
		var vectorData []byte
		if err := rows.Scan(&rec.ID, &rec.Context, &vectorData, &rec.SyncStatus, &lastSync); err != nil {
			return nil, err
		}
		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}
		records = append(records, rec)
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

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = $1
	`

	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, id); err != nil {
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	recordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`

	for _, rec := range records {

		vectorJSON, _ := json.Marshal(rec.Vector)
		if _, err := tx.Exec(ctx, query, rec.ID, rec.Context, string(vectorJSON), rec.SyncStatus); err != nil {
			syncErrorsTotal.Add(ctx, 1)
			return err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrorsTotal.Add(ctx, 1)
		return err
	}

	return nil
}
