package autodream

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// EmbeddingService represents a service to generate vector embeddings.
type EmbeddingService interface {
	EmbedText(ctx context.Context, text string) ([]float32, error)
}

// Memory struct represents a row in autodream_memories.
type Memory struct {
	ID        string
	TaskID    *string
    AgentID   string
    MemoryType string
	Content   string
	Embedding []float32
	CreatedAt time.Time
}

// Consolidator consolidates completed tasks into long-term memory.
type Consolidator struct {
	DB               db.Provider
	EmbeddingService EmbeddingService
}

// NewConsolidator creates a new Consolidator pipeline.
func NewConsolidator(database db.Provider, embeddingService EmbeddingService) *Consolidator {
	return &Consolidator{
		DB:               database,
		EmbeddingService: embeddingService,
	}
}

// ProcessCompletedTask takes a completed task, generates an embedding for it,
// and saves it to the vector database.
func (c *Consolidator) ProcessCompletedTask(ctx context.Context, task *orchestration.SharedTaskDecomposition) error {
	if task.Status != "DONE" {
		return fmt.Errorf("task %s is not DONE", task.ID)
	}

	content := task.Title
	if task.Description != nil {
		content += "\n" + *task.Description
	}
    if len(task.Payload) > 0 && string(task.Payload) != "{}" {
        content += "\nPayload: " + string(task.Payload)
    }

	embedding, err := c.EmbeddingService.EmbedText(ctx, content)
	if err != nil {
		return fmt.Errorf("failed to generate embedding: %w", err)
	}

    agentID := "unknown"
    if task.AssignedAgentID != nil {
        agentID = *task.AssignedAgentID
    }

	memory := &Memory{
		ID:         uuid.New().String(),
		TaskID:     &task.ID,
        AgentID:    agentID,
        MemoryType: "task_completion",
		Content:    content,
		Embedding:  embedding,
		CreatedAt:  time.Now(),
	}

	return c.saveMemory(ctx, memory)
}

func (c *Consolidator) saveMemory(ctx context.Context, memory *Memory) error {
    embeddingStr, err := json.Marshal(memory.Embedding)
    if err != nil {
        return err
    }

    if c.DB.IsSQLite() {
		// SQLite might not have vector extension, fallback gracefully or just ignore embedding for tests.
		// Since we use modernc.org/sqlite, it doesn't support vector. We store embedding as JSON string.
		query := `INSERT INTO autodream_memories_master (id, task_id, agent_id, memory_type, content, embedding, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)`
		_, err := c.DB.Exec(ctx, query, memory.ID, memory.TaskID, memory.AgentID, memory.MemoryType, memory.Content, embeddingStr, memory.CreatedAt)
		return err
	}

	query := `
		INSERT INTO autodream_memories_master (id, task_id, agent_id, memory_type, content, embedding, created_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`

	_, err = c.DB.Exec(ctx, query, memory.ID, memory.TaskID, memory.AgentID, memory.MemoryType, memory.Content, embeddingStr, memory.CreatedAt)
	return err
}
