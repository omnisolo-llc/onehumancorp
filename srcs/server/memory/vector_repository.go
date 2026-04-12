package memory

import (
	"context"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type Memory struct {
	ID           string
	TenantID     string
	MemoryType   string
	Content      string
	Embedding    []float32
	SourceTaskID string
}

type VectorRepository interface {
	Upsert(ctx context.Context, mem Memory) error
}

type PostgresVectorRepository struct {
	db db.Provider
}

func NewVectorRepository(db db.Provider) VectorRepository {
	return &PostgresVectorRepository{db: db}
}

func formatFloat32SliceForVector(embedding []float32) string {
	if len(embedding) == 0 {
		return "[]"
	}
	strs := make([]string, len(embedding))
	for i, v := range embedding {
		strs[i] = fmt.Sprintf("%f", v)
	}
	return "[" + strings.Join(strs, ",") + "]"
}

func (r *PostgresVectorRepository) Upsert(ctx context.Context, mem Memory) error {
	embStr := formatFloat32SliceForVector(mem.Embedding)
	var query string
	if r.db.IsSQLite() {
		query = `INSERT INTO ohc_memory_embeddings (id, tenant_id, memory_type, content, embedding, source_task_id, created_at)
			VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP)
			ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding`
	} else {
		query = `INSERT INTO ohc_memory_embeddings (id, tenant_id, memory_type, content, embedding, source_task_id, created_at)
			VALUES ($1, $2, $3, $4, $5::vector, $6, CURRENT_TIMESTAMP)
			ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding`
	}

	_, err := r.db.Exec(ctx, query, mem.ID, mem.TenantID, mem.MemoryType, mem.Content, embStr, mem.SourceTaskID)
	return err
}
