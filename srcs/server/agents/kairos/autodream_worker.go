package kairos

import (
	"context"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamWorker struct {
	db     db.Provider
	client LLMClient
}

func NewAutoDreamWorker(db db.Provider, client LLMClient) *AutoDreamWorker {
	return &AutoDreamWorker{
		db:     db,
		client: client,
	}
}

func (w *AutoDreamWorker) RunConsolidationPipeline(ctx context.Context) error {
	slog.Info("AutoDreamWorker: starting task consolidation pipeline")

	tx, err := w.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var rows db.Rows

	if w.db.IsSQLite() {
		rows, err = tx.Query(ctx, "SELECT id, payload, description, title FROM shared_tasks_decomposition WHERE status = 'COMPLETED' AND id NOT IN (SELECT task_id FROM autodream_memories)")
	} else {
		rows, err = tx.Query(ctx, "SELECT id, payload, description, title FROM shared_tasks_decomposition WHERE status = 'COMPLETED' AND id NOT IN (SELECT task_id FROM autodream_memories) FOR UPDATE SKIP LOCKED")
	}

	if err != nil {
		return fmt.Errorf("failed to query completed tasks: %w", err)
	}
	defer rows.Close()

	type Task struct {
		ID          string
		Payload     []byte
		Description string
		Title       string
	}

	var tasks []Task
	for rows.Next() {
		var t Task
		if err := rows.Scan(&t.ID, &t.Payload, &t.Description, &t.Title); err == nil {
			tasks = append(tasks, t)
		}
	}
	rows.Close()

	for _, task := range tasks {
		content := fmt.Sprintf("Title: %s\nDescription: %s\nPayload: %s", task.Title, task.Description, string(task.Payload))

		embedding := make([]float32, 1536)
		if w.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embedErr := w.client.GenerateEmbedding(ctxTimeout, content)
			cancel()
			if embedErr == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Warn("AutoDreamWorker: failed to generate embedding", "error", embedErr)
				continue // Skip insertion if embedding generation fails
			}
		}

		embStrs := make([]string, len(embedding))
		for i, v := range embedding {
			embStrs[i] = fmt.Sprintf("%f", v)
		}
		embStr := "[" + strings.Join(embStrs, ",") + "]"

		var insertQuery string
		if w.db.IsSQLite() {
			insertQuery = "INSERT INTO autodream_memories (task_id, content, embedding) VALUES ($1, $2, $3)"
		} else {
			insertQuery = "INSERT INTO autodream_memories (task_id, content, embedding) VALUES ($1, $2, $3::vector)"
		}

		// Also we must generate an ID since the sqlite test doesn't use gen_random_uuid
		if w.db.IsSQLite() {
			insertQuery = "INSERT INTO autodream_memories (id, task_id, content, embedding) VALUES (lower(hex(randomblob(16))), $1, $2, $3)"
		}

		if _, err := tx.Exec(ctx, insertQuery, task.ID, content, embStr); err != nil {
			slog.Error("AutoDreamWorker: failed to insert memory", "task_id", task.ID, "error", err)
			continue
		}
		slog.Info("AutoDreamWorker: consolidated memory", "task_id", task.ID)
	}

	return tx.Commit(ctx)
}

// StartWorkerDaemon creates a background loop that processes pending completed tasks for AutoDream embedding.
func (w *AutoDreamWorker) StartWorkerDaemon(ctx context.Context) {
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := w.RunConsolidationPipeline(ctx); err != nil {
				slog.Error("AutoDreamWorker daemon encountered error", "error", err)
			}
		}
	}
}
