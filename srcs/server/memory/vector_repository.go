package memory

import (
    "context"
    "encoding/json"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type OHCMemoryEmbedding struct {
    ID           string
    TenantID     string
    MemoryType   string
    Content      string
    Embedding    []float32
    SourceTaskID string
    CreatedAt    time.Time
}

type VectorRepository interface {
    UpsertEmbedding(ctx context.Context, mem *OHCMemoryEmbedding) error
    SemanticSearch(ctx context.Context, tenantID string, queryEmbedding []float32, limit int) ([]*OHCMemoryEmbedding, error)
}

type PostgresVectorRepository struct {
    db db.Provider
}

func NewVectorRepository(provider db.Provider) VectorRepository {
    return &PostgresVectorRepository{db: provider}
}

func (r *PostgresVectorRepository) UpsertEmbedding(ctx context.Context, mem *OHCMemoryEmbedding) error {
    embeddingStr := "[0.0]"
    if len(mem.Embedding) > 0 {
        bytes, err := json.Marshal(mem.Embedding)
        if err == nil {
            embeddingStr = string(bytes)
        }
    }

    var query string
    var args []interface{}

    if r.db.IsSQLite() {
        query = `
            INSERT INTO ohc_memory_embeddings (id, tenant_id, memory_type, content, embedding, source_task_id, created_at)
            VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
        `
        args = []interface{}{mem.ID, mem.TenantID, mem.MemoryType, mem.Content, embeddingStr, mem.SourceTaskID}
    } else {
        query = `
            INSERT INTO ohc_memory_embeddings (id, tenant_id, memory_type, content, embedding, source_task_id, created_at)
            VALUES ($1, $2, $3, $4, $5::vector, $6, NOW())
            ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
        `
        args = []interface{}{mem.ID, mem.TenantID, mem.MemoryType, mem.Content, embeddingStr, mem.SourceTaskID}
    }

    _, err := r.db.Exec(ctx, query, args...)
    return err
}

func (r *PostgresVectorRepository) SemanticSearch(ctx context.Context, tenantID string, queryEmbedding []float32, limit int) ([]*OHCMemoryEmbedding, error) {
    // Graceful degradation for SQLite - just return recent
    if r.db.IsSQLite() {
        query := `
            SELECT id, tenant_id, memory_type, content, source_task_id, created_at
            FROM ohc_memory_embeddings
            WHERE tenant_id = ?
            ORDER BY created_at DESC
            LIMIT ?
        `
        rows, err := r.db.Query(ctx, query, tenantID, limit)
        if err != nil {
            return nil, err
        }
        defer rows.Close()

        var results []*OHCMemoryEmbedding
        for rows.Next() {
            var m OHCMemoryEmbedding
            if err := rows.Scan(&m.ID, &m.TenantID, &m.MemoryType, &m.Content, &m.SourceTaskID, &m.CreatedAt); err == nil {
                results = append(results, &m)
            }
        }
        return results, nil
    }

    embeddingStr := "[0.0]"
    if len(queryEmbedding) > 0 {
        bytes, err := json.Marshal(queryEmbedding)
        if err == nil {
            embeddingStr = string(bytes)
        }
    }

    query := `
        SELECT id, tenant_id, memory_type, content, source_task_id, created_at
        FROM ohc_memory_embeddings
        WHERE tenant_id = $1
        ORDER BY embedding <-> $2::vector
        LIMIT $3
    `
    rows, err := r.db.Query(ctx, query, tenantID, embeddingStr, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var results []*OHCMemoryEmbedding
    for rows.Next() {
        var m OHCMemoryEmbedding
        if err := rows.Scan(&m.ID, &m.TenantID, &m.MemoryType, &m.Content, &m.SourceTaskID, &m.CreatedAt); err == nil {
            results = append(results, &m)
        }
    }
    return results, nil
}
