package hub

import (
    "context"
    "encoding/binary"
    "fmt"
    "math"
    "time"
    "github.com/onehumancorp/mono/srcs/server/db"
    "database/sql"
    "strings"
)

type ragSyncServiceImpl struct {
    provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
    return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    query := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1`
    if s.provider.IsSQLite() {
        query = `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT ?`
    }

    rows, err := s.provider.Query(ctx, query, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var records []RAGSyncRecord
    for rows.Next() {
        var rec RAGSyncRecord
        var vecBytes []byte
        var lastSync sql.NullString
        if err := rows.Scan(&rec.ID, &rec.Context, &vecBytes, &rec.SyncStatus, &lastSync); err != nil {
            return nil, err
        }

        if len(vecBytes) > 0 {
            floats := make([]float32, len(vecBytes)/4)
            for i := 0; i < len(floats); i++ {
                bits := binary.LittleEndian.Uint32(vecBytes[i*4 : (i+1)*4])
                floats[i] = math.Float32frombits(bits)
            }
            rec.Vector = floats
        }

        if lastSync.Valid {
            t, err := time.Parse(time.RFC3339, lastSync.String)
            if err == nil {
                rec.LastSyncAt = t
            }
        }

        records = append(records, rec)
    }
    return records, rows.Err()
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
    if len(ids) == 0 {
        return nil
    }

    placeholders := make([]string, len(ids))
    args := make([]any, len(ids))
    for i, id := range ids {
        if s.provider.IsSQLite() {
            placeholders[i] = "?"
        } else {
            placeholders[i] = fmt.Sprintf("$%d", i+1)
        }
        args[i] = id
    }

    query := fmt.Sprintf(`UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id IN (%s)`, strings.Join(placeholders, ","))
    _, err := s.provider.Exec(ctx, query, args...)
    if err != nil && RagSyncErrorsTotal != nil {
        RagSyncErrorsTotal.Add(ctx, 1)
    } else if err == nil && RagRecordsSyncedTotal != nil {
        RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
    }
    return err
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    for _, rec := range records {
        var vecBytes []byte
        if len(rec.Vector) > 0 {
            vecBytes = make([]byte, len(rec.Vector)*4)
            for i, f := range rec.Vector {
                binary.LittleEndian.PutUint32(vecBytes[i*4:(i+1)*4], math.Float32bits(f))
            }
        }

        query := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                  VALUES ($1, $2, $3, $4, $5)
                  ON CONFLICT (memory_id) DO UPDATE
                  SET context = EXCLUDED.context,
                      vector_embedding = EXCLUDED.vector_embedding,
                      sync_status = EXCLUDED.sync_status,
                      last_sync_at = EXCLUDED.last_sync_at`

        if s.provider.IsSQLite() {
            query = `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
                     VALUES (?, ?, ?, ?, ?)
                     ON CONFLICT (memory_id) DO UPDATE
                     SET context = EXCLUDED.context,
                         vector_embedding = EXCLUDED.vector_embedding,
                         sync_status = EXCLUDED.sync_status,
                         last_sync_at = EXCLUDED.last_sync_at`
        }

        var lastSyncAtAny any
        if rec.LastSyncAt.IsZero() {
            lastSyncAtAny = nil
        } else {
            lastSyncAtAny = rec.LastSyncAt.Format(time.RFC3339)
        }

        _, err := s.provider.Exec(ctx, query, rec.ID, rec.Context, vecBytes, rec.SyncStatus, lastSyncAtAny)
        if err != nil {
            if RagSyncErrorsTotal != nil {
                RagSyncErrorsTotal.Add(ctx, 1)
            }
            return err
        }
    }

    if RagRecordsSyncedTotal != nil {
        RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
    }
    return nil
}
