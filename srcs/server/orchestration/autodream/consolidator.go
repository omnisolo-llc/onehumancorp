package autodream

import (
	"context"
	"fmt"
	"time"


)

type EmbeddingService interface {
	GetEmbedding(ctx context.Context, text string) ([]float32, error)
}

type Consolidator struct {
	Repo             *Repository
	EmbeddingService EmbeddingService
}

func NewConsolidator(repo *Repository, embeddingService EmbeddingService) *Consolidator {
	return &Consolidator{
		Repo:             repo,
		EmbeddingService: embeddingService,
	}
}

func (c *Consolidator) ProcessCompletedTasks(ctx context.Context) error {
	// 1. Fetch completed tasks that haven't been consolidated
	query := `
		SELECT std.id, std.name, std.description, std.output_payload, std.organization_id
		FROM shared_tasks_decomposition std
		LEFT JOIN autodream_memories am ON std.id = am.task_id
		WHERE std.state = 'DONE' AND am.task_id IS NULL
		LIMIT 50
	`

	rows, err := c.Repo.Provider.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to fetch completed tasks: %w", err)
	}
	defer rows.Close()

	type taskData struct {
		ID             string
		Name           string
		Description    string
		OutputPayload  *string
		OrganizationID string
	}
	var tasksToProcess []taskData

	for rows.Next() {
		var t taskData
		var desc *string
		if err := rows.Scan(&t.ID, &t.Name, &desc, &t.OutputPayload, &t.OrganizationID); err != nil {
			return fmt.Errorf("failed to scan task: %w", err)
		}
		if desc != nil {
			t.Description = *desc
		}
		tasksToProcess = append(tasksToProcess, t)
	}

	// 2. Generate embeddings and insert into autodream_memories
	for _, t := range tasksToProcess {
		content := fmt.Sprintf("Task Name: %s\nDescription: %s\n", t.Name, t.Description)
		if t.OutputPayload != nil {
			content += fmt.Sprintf("Output: %s\n", *t.OutputPayload)
		}

		embedding, err := c.EmbeddingService.GetEmbedding(ctx, content)
		if err != nil {
			// Log error and continue to next task
			fmt.Printf("Failed to get embedding for task %s: %v\n", t.ID, err)
			continue
		}

		memID := fmt.Sprintf("ad_mem_%s_%d", t.ID, time.Now().UnixNano())

		mem := &Memory{
			ID:             memID,
			TaskID:         t.ID,
			Content:        content,
			Embedding:      embedding,
			OrganizationID: t.OrganizationID,
		}

		if err := c.Repo.Insert(ctx, mem); err != nil {
			fmt.Printf("Failed to insert memory for task %s: %v\n", t.ID, err)
			continue
		}
	}

	return nil
}
