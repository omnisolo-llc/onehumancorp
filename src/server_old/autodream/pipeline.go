package autodream

import (
	"context"
	"fmt"
	"log/slog"
	"strings"

	"github.com/onehumancorp/mono/src/server/db"
)

// LLMClient provides embedding capabilities
type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// AutoDreamPipeline polls completed tasks and generates embeddings to store as memories.
type AutoDreamPipeline struct {
	pool db.Provider
	llm  LLMClient
}

func NewAutoDreamPipeline(pool db.Provider, llm LLMClient) *AutoDreamPipeline {
	return &AutoDreamPipeline{
		pool: pool,
		llm:  llm,
	}
}

// ProcessCompletedTasks polls for completed shared_tasks_decomposition and generates embeddings.
func (p *AutoDreamPipeline) ProcessCompletedTasks(ctx context.Context) error {
	query := `
		SELECT id, organization_id, COALESCE(description, title, '') AS content
		FROM shared_tasks_decomposition
		WHERE status = 'COMPLETED'
		  AND id NOT IN (SELECT task_id FROM autodream_memories WHERE task_id IS NOT NULL)
		LIMIT 100
	`

	rows, err := p.pool.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query tasks: %w", err)
	}
	defer rows.Close()

	type taskEntry struct {
		ID      string
		OrgID   string
		Content string
	}
	var tasks []taskEntry

	for rows.Next() {
		var t taskEntry
		if err := rows.Scan(&t.ID, &t.OrgID, &t.Content); err != nil {
			slog.Error("failed to scan task", "error", err)
			continue
		}
		if t.Content == "" {
			t.Content = "No content provided."
		}
		tasks = append(tasks, t)
	}

	if err := rows.Err(); err != nil {
		return fmt.Errorf("row iteration error: %w", err)
	}

	for _, t := range tasks {
		var embStr string
		embedding, err := p.llm.GenerateEmbedding(ctx, t.Content)
		if err != nil {
			slog.Warn("failed to generate embedding, skipping task for retry later", "task_id", t.ID, "error", err)
			continue
		}

		strs := make([]string, len(embedding))
		for i, v := range embedding {
			strs[i] = fmt.Sprintf("%f", v)
		}
		embStr = "[" + strings.Join(strs, ",") + "]"

		memID := "mem-" + t.ID
		insertQuery := `
			INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type)
			VALUES ($1, $2, $3, $4, $5, 'task')
		`
		if p.pool.IsSQLite() {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type)
				VALUES (?, ?, ?, ?, ?, 'task')
			`
		}

		_, err = p.pool.Exec(ctx, insertQuery, memID, t.OrgID, t.ID, t.Content, embStr)
		if err != nil {
			slog.Error("failed to insert memory", "task_id", t.ID, "error", err)
		} else {
			slog.Info("ingested completed task", "task_id", t.ID)
		}
	}

	return nil
}
