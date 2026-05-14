package orchestration

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pgvector/pgvector-go"
)

type SwarmMemoryEmbedding struct {
	MemoryID        string
	Context         string
	VectorEmbedding []float32
	SourcePlugin    string
	OrganizationID  string
}

type AutoDreamPipeline struct {
	db *sql.DB
}

func NewAutoDreamPipeline(db *sql.DB) *AutoDreamPipeline {
	return &AutoDreamPipeline{
		db: db,
	}
}

func (p *AutoDreamPipeline) InsertEmbedding(ctx context.Context, memory SwarmMemoryEmbedding) error {
	if len(memory.VectorEmbedding) != 1536 {
		return fmt.Errorf("vector embedding must have exactly 1536 dimensions")
	}

	query := `
		INSERT INTO swarm_memory_embeddings (
			memory_id, context, vector_embedding, source_plugin, organization_id
		) VALUES ($1, $2, $3, $4, $5)
	`

    vectorValue := pgvector.NewVector(memory.VectorEmbedding)

	_, err := p.db.ExecContext(ctx, query, memory.MemoryID, memory.Context, vectorValue, memory.SourcePlugin, memory.OrganizationID)
	return err
}

func (p *AutoDreamPipeline) SearchSimilarity(ctx context.Context, organizationID string, embedding []float32, limit int) ([]SwarmMemoryEmbedding, error) {
	if len(embedding) != 1536 {
		return nil, fmt.Errorf("vector embedding must have exactly 1536 dimensions")
	}

	query := `
		SELECT memory_id, context, vector_embedding, source_plugin, organization_id
		FROM swarm_memory_embeddings
		WHERE organization_id = $1
		ORDER BY vector_embedding <=> $2
		LIMIT $3
	`

    vectorValue := pgvector.NewVector(embedding)
	rows, err := p.db.QueryContext(ctx, query, organizationID, vectorValue, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []SwarmMemoryEmbedding
	for rows.Next() {
		var mem SwarmMemoryEmbedding
		var vec pgvector.Vector
		if err := rows.Scan(&mem.MemoryID, &mem.Context, &vec, &mem.SourcePlugin, &mem.OrganizationID); err != nil {
			return nil, err
		}
		mem.VectorEmbedding = vec.Slice()
		results = append(results, mem)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}

	return results, nil
}
