package hub

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

    "github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/telemetry"
)

type ragSyncService struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncService{provider: provider}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT memory_id, context, vector_embedding, sync_status FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        telemetry.RecordRAGSyncError(ctx, err.Error())
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var r RAGSyncRecord
        var vecBytes []byte
        if err := rows.Scan(&r.ID, &r.Context, &vecBytes, &r.SyncStatus); err != nil {
            telemetry.RecordRAGSyncError(ctx, err.Error())
            return nil, err
        }
        if len(vecBytes) > 0 {
            _ = json.Unmarshal(vecBytes, &r.Vector)
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

    args := make([]any, len(ids)+1)
    args[0] = time.Now()
    placeholders := make([]string, len(ids))
    for i, id := range ids {
        args[i+1] = id
        placeholders[i] = fmt.Sprintf("$%d", i+2)
    }

    query := fmt.Sprintf("UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id IN (%s)", strings.Join(placeholders, ", "))

    res, err := s.provider.Exec(ctx, query, args...)
    if err != nil {
        telemetry.RecordRAGSyncError(ctx, err.Error())
        return err
    }

    if res > 0 {
        telemetry.RecordRAGRecordsSynced(ctx, res)
    }
    return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    tx, err := s.provider.Begin(ctx)
    if err != nil {
        telemetry.RecordRAGSyncError(ctx, err.Error())
        return err
    }
    defer tx.Rollback(ctx)

    var successCount int64
    var vectorCount int64

    for _, r := range records {
        var count int
        err := tx.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = $1", r.ID).Scan(&count)
        if err != nil {
            telemetry.RecordRAGSyncError(ctx, err.Error())
            return err
        }

        var vecBytes []byte
        if len(r.Vector) > 0 {
            vectorCount++
            vecBytes, _ = json.Marshal(r.Vector)
        }

        if count > 0 {
            _, err = tx.Exec(ctx, "UPDATE swarm_memory_embeddings SET context = $1, vector_embedding = $2, sync_status = $3, last_sync_at = $4 WHERE memory_id = $5", r.Context, vecBytes, r.SyncStatus, r.LastSyncAt, r.ID)
        } else {
            _, err = tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, $4, $5)", r.ID, r.Context, vecBytes, r.SyncStatus, r.LastSyncAt)
        }
        if err != nil {
            telemetry.RecordRAGSyncError(ctx, err.Error())
            return err
        }
        successCount++
    }

    if err := tx.Commit(ctx); err != nil {
        telemetry.RecordRAGSyncError(ctx, err.Error())
        return err
    }

    if successCount > 0 {
        telemetry.RecordRAGRecordsSynced(ctx, successCount)
    }
    if vectorCount > 0 && telemetry.VectorsSyncedCount != nil {
        telemetry.VectorsSyncedCount.Add(ctx, vectorCount)
    }
    return nil
}
