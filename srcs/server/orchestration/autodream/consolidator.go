package autodream

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type EmbeddingService interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
	GenerateSummary(ctx context.Context, payload string, logs string) (string, error)
}

type Consolidator struct {
	db       db.Provider
	embedder EmbeddingService
}

func NewConsolidator(database db.Provider, embedder EmbeddingService) *Consolidator {
	return &Consolidator{
		db:       database,
		embedder: embedder,
	}
}

type CompletedTask struct {
	ID             string
	OrganizationID string
	Payload        string
	Logs           string
}

func (c *Consolidator) ProcessCompletedTask(ctx context.Context, task CompletedTask) error {
	summary, err := c.embedder.GenerateSummary(ctx, task.Payload, task.Logs)
	if err != nil {
		return fmt.Errorf("failed to generate summary: %w", err)
	}

	embedding, err := c.embedder.GenerateEmbedding(ctx, summary)
	if err != nil {
		return fmt.Errorf("failed to generate embedding: %w", err)
	}

	return c.insertMemory(ctx, task.OrganizationID, task.ID, summary, embedding, map[string]interface{}{
		"original_payload_length": len(task.Payload),
		"logs_length":             len(task.Logs),
	})
}

// Float32SliceToVectorString converts a []float32 to the pgvector string format '[1.0, 2.0, ...]'
func Float32SliceToVectorString(vec []float32) string {
	b, _ := json.Marshal(vec)
	return string(b)
}

func (c *Consolidator) insertMemory(ctx context.Context, orgID string, taskID string, content string, embedding []float32, metadata map[string]interface{}) error {
	metaJSON, err := json.Marshal(metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	query := `
		INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, metadata)
		VALUES ($1, $2, $3, $4, $5, $6)
	`
	id := uuid.New().String()

	vectorStr := Float32SliceToVectorString(embedding)

	_, err = c.db.Exec(ctx, query, id, orgID, taskID, content, vectorStr, metaJSON)
	return err
}
