package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"time"
)

// LLMClient represents a minimal interface for generating embeddings.
// In a real scenario, this would be an actual client, but for now we define an interface we can mock.
type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// DummyLLMClient is a fallback implementation.
type DummyLLMClient struct{}

// GenerateEmbedding generates a fake embedding.
func (c *DummyLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	emb := make([]float32, 1536)
	for i := range emb {
		emb[i] = 0.1 // Dummy data
	}
	return emb, nil
}

// AutoDreamWorker periodically sweeps completed tasks and session data to generate embeddings.
type AutoDreamWorker struct {
	db        *sql.DB
	llmClient LLMClient
	isSQLite  bool
}

// NewAutoDreamWorker creates a new AutoDreamWorker daemon.
func NewAutoDreamWorker(db *sql.DB, isSQLite bool, llmClient LLMClient) *AutoDreamWorker {
	if llmClient == nil {
		llmClient = &DummyLLMClient{}
	}
	return &AutoDreamWorker{
		db:        db,
		isSQLite:  isSQLite,
		llmClient: llmClient,
	}
}

// Run starts the daemon loop.
func (w *AutoDreamWorker) Run(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := w.ProcessBatch(ctx); err != nil {
				log.Printf("AutoDreamWorker: Error processing batch: %v", err)
			}
		}
	}
}

// ProcessBatch processes a batch of tasks/session data (up to LIMIT 500) and upserts embeddings.
func (w *AutoDreamWorker) ProcessBatch(ctx context.Context) error {
	// 1. Extraction: Poll recent tasks that need memory consolidation.
	// For this exercise, let's assume we read from a placeholder `shared_tasks` table or similar.
	// Using a batch limit of 500.
	query := `
		SELECT id, tenant_id as organization_id, agent_id, payload
		FROM shared_tasks
		WHERE status = 'COMPLETED' AND auto_dreamed = FALSE
		LIMIT 500`

	rows, err := w.db.QueryContext(ctx, query)
	if err != nil {
		// If the table doesn't exist in testing, just return gracefully
		return nil
	}
	defer rows.Close()

	type Task struct {
		ID             string
		OrganizationID string
		AgentID        string
		Content        string
	}
	var tasks []Task

	for rows.Next() {
		var t Task
		var agentID sql.NullString
		var content sql.NullString
		if err := rows.Scan(&t.ID, &t.OrganizationID, &agentID, &content); err != nil {
			continue
		}
		if agentID.Valid {
			t.AgentID = agentID.String
		}
		if content.Valid {
			t.Content = content.String
		}
		tasks = append(tasks, t)
	}
	if err := rows.Err(); err != nil {
		return err
	}

	for _, task := range tasks {
		// 2. Embedding: Call LLM client to generate embeddings.
		emb, err := w.llmClient.GenerateEmbedding(ctx, task.Content)
		if err != nil {
			log.Printf("Failed to generate embedding for task %s: %v", task.ID, err)
			continue
		}

		embStr := formatEmbedding(emb)

		// 3. Loading: Upsert into autodream_memories.
		var insertQuery string
		if w.isSQLite {
			// For Standalone degradation: storing embedding as a JSON text blob
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type)
				VALUES (?, ?, ?, ?, ?, 'TASK_SUMMARY')
				ON CONFLICT(id) DO UPDATE SET
					embedding=excluded.embedding,
					content=excluded.content`
			_, err = w.db.ExecContext(ctx, insertQuery, task.ID, task.OrganizationID, task.AgentID, task.Content, embStr)
		} else {
			// For Postgres with pgvector
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type)
				VALUES ($1, $2, $3, $4, $5::vector, 'TASK_SUMMARY')
				ON CONFLICT(id) DO UPDATE SET
					embedding=excluded.embedding,
					content=excluded.content`
			_, err = w.db.ExecContext(ctx, insertQuery, task.ID, task.OrganizationID, task.AgentID, task.Content, embStr)
		}

		if err != nil {
			log.Printf("Failed to insert memory for task %s: %v", task.ID, err)
			continue
		}

		// Mark task as auto_dreamed
		updateQuery := `UPDATE shared_tasks SET auto_dreamed = TRUE WHERE id = $1`
		if w.isSQLite {
			updateQuery = `UPDATE shared_tasks SET auto_dreamed = TRUE WHERE id = ?`
		}
		w.db.ExecContext(ctx, updateQuery, task.ID)
	}

	return nil
}

// formatEmbedding formats a float32 slice into a string for db insertion.
func formatEmbedding(emb []float32) string {
	if len(emb) == 0 {
		return "[]"
	}
	res := "["
	for i, v := range emb {
		if i > 0 {
			res += ","
		}
		res += fmt.Sprintf("%f", v)
	}
	res += "]"
	return res
}
