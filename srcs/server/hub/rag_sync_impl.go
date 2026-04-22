package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ragSyncService struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{provider: provider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, err.Error())
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorBytes []byte
		if err := rows.Scan(&r.ID, &r.Context, &vectorBytes, &r.SyncStatus, &r.LastSyncAt); err != nil {
			telemetry.RecordRAGSyncError(ctx, err.Error())
			return nil, err
		}
		if len(vectorBytes) > 0 {
			_ = json.Unmarshal(vectorBytes, &r.Vector)
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		telemetry.RecordRAGSyncError(ctx, err.Error())
		return records, err
	}
	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Use batch update with IN clause for efficiency
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids)+1)
	args[0] = time.Now()
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+2)
		args[i+1] = id
	}

	query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id IN (%s)", strings.Join(placeholders, ","))
	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx, err.Error())
		return err
	}

	telemetry.RecordRAGRecordsSynced(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	var successCount int64
	for _, r := range records {
		vectorBytes, _ := json.Marshal(r.Vector)

		var query string
		var args []interface{}

		if s.provider.IsSQLite() {
			// SQLite UPSERT
			query = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4, $5)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
			args = []interface{}{r.ID, r.Context, vectorBytes, r.SyncStatus, r.LastSyncAt}
		} else {
			// Postgres UPSERT
			query = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4, $5)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = EXCLUDED.context,
					vector_embedding = EXCLUDED.vector_embedding,
					sync_status = EXCLUDED.sync_status,
					last_sync_at = EXCLUDED.last_sync_at
			`
			args = []interface{}{r.ID, r.Context, vectorBytes, string(r.SyncStatus), r.LastSyncAt}
		}

		_, err := s.provider.Exec(ctx, query, args...)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx, err.Error())
			return err
		}
		successCount++
	}

	if successCount > 0 {
		telemetry.RecordRAGRecordsSynced(ctx, successCount)
	}
	return nil
}
