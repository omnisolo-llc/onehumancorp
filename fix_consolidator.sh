cat << 'GO' > srcs/server/orchestration/autodream/consolidator.go
package autodream

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// EmbeddingClient generates vector embeddings for text
type EmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// Consolidator runs the background pipeline for memory consolidation
type Consolidator struct {
	db     db.Provider
	client EmbeddingClient
}

// NewConsolidator creates a new Consolidator instance
func NewConsolidator(provider db.Provider, client EmbeddingClient) *Consolidator {
	return &Consolidator{
		db:     provider,
		client: client,
	}
}

// Consolidate fetches completed tasks using SKIP LOCKED to avoid concurrent pod collisions,
// and processes them into autodream_memories.
func (c *Consolidator) Consolidate(ctx context.Context) error {
	slog.Info("Starting autoDream consolidation sweep")

	// 1. Fetch completed tasks that haven't been consolidated yet, using FOR UPDATE SKIP LOCKED
	// Also use NOT EXISTS instead of NOT IN to avoid the NULL trap.
	query := `
		SELECT id, organization_id, payload
		FROM shared_tasks_decomposition
		WHERE status = 'COMPLETED'
		AND NOT EXISTS (
			SELECT 1 FROM autodream_memories WHERE task_id = shared_tasks_decomposition.id
		)
	`
	if !c.db.IsSQLite() {
		query += ` FOR UPDATE SKIP LOCKED`
	}
	query += ` LIMIT 50`

	tx, err := c.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query completed tasks: %w", err)
	}
	defer rows.Close()

	type TaskData struct {
		ID             string
		OrganizationID string
		Payload        string
	}
	var tasks []TaskData

	for rows.Next() {
		var td TaskData
		var payloadJSON []byte
		if err := rows.Scan(&td.ID, &td.OrganizationID, &payloadJSON); err != nil {
			slog.Error("Failed to scan task row", "error", err)
			continue
		}
		td.Payload = string(payloadJSON)
		tasks = append(tasks, td)
	}
	rows.Close() // Must close rows before executing inserts on the same tx

	for _, task := range tasks {
		slog.Info("Consolidating task", "task_id", task.ID)

		// 2. Generate summary/embedding (using payload as content)
		content := task.Payload
		if content == "" {
			content = "Task completed without payload."
		}

		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		embedding, err := c.client.GenerateEmbedding(ctxTimeout, content)
		cancel()

		if err != nil {
			slog.Error("Failed to generate embedding", "task_id", task.ID, "error", err)
			continue
		}

		embeddingStr := "[0.0]"
		if len(embedding) > 0 {
			if bytes, err := json.Marshal(embedding); err == nil {
				embeddingStr = string(bytes)
			}
		}

		// 3. Insert into autodream_memories
		memID := "ad_" + task.ID + "_" + fmt.Sprintf("%d", time.Now().UnixNano())

		var insertQuery string
		var insertArgs []interface{}

		if c.db.IsSQLite() {
		    // For local testing compatibility, we use TEXT for embedding
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding)
				VALUES (?, ?, ?, ?, ?)
			`
			insertArgs = []interface{}{memID, task.OrganizationID, task.ID, content, embeddingStr}
		} else {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding)
				VALUES ($1, $2, $3, $4, $5::vector)
			`
			insertArgs = []interface{}{memID, task.OrganizationID, task.ID, content, embeddingStr}
		}

		if _, err := tx.Exec(ctx, insertQuery, insertArgs...); err != nil {
			slog.Error("Failed to insert consolidated memory", "task_id", task.ID, "error", err)
		} else {
			slog.Info("Successfully consolidated task into memory", "task_id", task.ID, "memory_id", memID)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	slog.Info("Completed autoDream consolidation sweep", "processed", len(tasks))
	return nil
}
GO
git add srcs/server/orchestration/autodream/consolidator.go
