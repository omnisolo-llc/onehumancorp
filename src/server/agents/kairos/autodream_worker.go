package kairos

import (
	"context"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
)

// LLMClient defines the interface required by AutoDreamWorker to generate embeddings.
type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// AutoDreamWorker represents the background worker for memory consolidation.
type AutoDreamWorker struct {
	pool   db.Provider
	client LLMClient
}

// NewAutoDreamWorker creates a new AutoDreamWorker instance.
func NewAutoDreamWorker(pool db.Provider, client LLMClient) *AutoDreamWorker {
	return &AutoDreamWorker{
		pool:   pool,
		client: client,
	}
}

// Start runs the worker in a loop.
func (w *AutoDreamWorker) Start(ctx context.Context) {
	slog.Info("Starting KAIROS AutoDreamWorker")
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.ProcessCompletedTasks(ctx)
		}
	}
}

// ProcessCompletedTasks polls for completed shared tasks and converts them into embeddings.
func (w *AutoDreamWorker) ProcessCompletedTasks(ctx context.Context) {
	// Query to fetch tasks that are COMPLETED or DONE, but not yet auto-dreamed.
	query := `SELECT id, organization_id, COALESCE(payload, '{}') AS payload
	          FROM shared_tasks_decomposition
	          WHERE status IN ('COMPLETED', 'DONE') AND (auto_dreamed = false OR auto_dreamed IS NULL)
	          LIMIT 100`

	rows, err := w.pool.Query(ctx, query)
	if err != nil {
		if strings.Contains(err.Error(), "no such column: auto_dreamed") || strings.Contains(err.Error(), "column \"auto_dreamed\" does not exist") {
			return
		}
		slog.Error("AutoDreamWorker: failed to query completed tasks", "error", err)
		return
	}
	defer rows.Close()

	var tasks []struct {
		ID             string
		OrganizationID string
		Payload        string
	}

	for rows.Next() {
		var t struct {
			ID             string
			OrganizationID string
			Payload        string
		}
		if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Payload); err != nil {
			slog.Error("AutoDreamWorker: failed to scan task", "error", err)
			continue
		}
		tasks = append(tasks, t)
	}
	rows.Close()

	if len(tasks) == 0 {
		return
	}

	for _, t := range tasks {
		summary := fmt.Sprintf("Summary of task payload: %s", t.Payload)

		var embedding []float32
		if w.client != nil {
			embCtx, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embErr := w.client.GenerateEmbedding(embCtx, summary)
			cancel()
			if embErr == nil && len(resp) == 1536 {
				embedding = resp
			} else if embErr != nil {
				slog.Error("AutoDreamWorker: failed to generate embedding", "task_id", t.ID, "error", embErr)
			}
		}

		if len(embedding) == 0 {
			// Fallback placeholder embedding if generation fails or no client is provided
			embedding = make([]float32, 1536)
			embedding[0] = 0.1
		}

		strs := make([]string, len(embedding))
		for i, v := range embedding {
			strs[i] = fmt.Sprintf("%f", v)
		}
		embStr := "[" + strings.Join(strs, ",") + "]"

		memID := uuid.New().String()

		// Insert into autodream_memories
		insertQuery := `INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding)
		               VALUES ($1, $2, $3, $4, $5)`
		_, err := w.pool.Exec(ctx, insertQuery, memID, t.OrganizationID, t.ID, summary, embStr)
		if err != nil {
			insertQueryPg := `INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding)
			               VALUES ($1, $2, $3, $4, $5::vector)`
			_, errPg := w.pool.Exec(ctx, insertQueryPg, memID, t.OrganizationID, t.ID, summary, embStr)
			if errPg != nil {
				slog.Error("AutoDreamWorker: failed to insert memory", "task_id", t.ID, "error", errPg)
				continue
			}
		}

		// Mark task as auto_dreamed
		updateTaskQuery := `UPDATE shared_tasks_decomposition SET auto_dreamed = true WHERE id = $1`
		if _, err := w.pool.Exec(ctx, updateTaskQuery, t.ID); err != nil {
			slog.Error("AutoDreamWorker: failed to update task auto_dreamed status", "task_id", t.ID, "error", err)
		} else {
			slog.Info("AutoDreamWorker: ingested completed task", "task_id", t.ID)
		}
	}
}
