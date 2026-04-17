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

	// Phase 1: Retrieve tasks using a short-lived transaction
	tx, err := w.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}

	var rows db.Rows
	if w.db.IsSQLite() {
		rows, err = tx.Query(ctx, "SELECT id, payload, description, title FROM shared_tasks_decomposition WHERE status = 'COMPLETED' AND id NOT IN (SELECT task_id FROM autodream_memories)")
	} else {
		rows, err = tx.Query(ctx, "SELECT id, payload, description, title FROM shared_tasks_decomposition WHERE status = 'COMPLETED' AND id NOT IN (SELECT task_id FROM autodream_memories) FOR UPDATE SKIP LOCKED")
	}

	if err != nil {
		tx.Rollback(ctx)
		return fmt.Errorf("failed to query completed tasks: %w", err)
	}

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

	// We commit the read lock because we shouldn't hold a DB transaction during an LLM call.
	// We'll rely on UPSERT / ON CONFLICT to avoid inserting the same memory twice when running embeddings concurrently.
	tx.Commit(ctx)

	// Phase 2: Generate Embeddings without holding a lock
	for _, task := range tasks {
		content := fmt.Sprintf("Title: %s\nDescription: %s\nPayload: %s", task.Title, task.Description, string(task.Payload))

		var embedding []float32
		if w.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embedErr := w.client.GenerateEmbedding(ctxTimeout, content)
			cancel()
			if embedErr == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Warn("AutoDreamWorker: failed to generate embedding", "error", embedErr)
				continue // Skip insertion if embedding generation fails to prevent data corruption
			}
		} else {
			// If no LLM, we should skip to prevent data corruption with zero arrays.
			slog.Warn("AutoDreamWorker: no LLM client configured, skipping")
			continue
		}

		embStrs := make([]string, len(embedding))
		for i, v := range embedding {
			embStrs[i] = fmt.Sprintf("%f", v)
		}
		embStr := "[" + strings.Join(embStrs, ",") + "]"

		// Phase 3: Insert using a new transaction
		insertTx, err := w.db.Begin(ctx)
		if err != nil {
			slog.Error("AutoDreamWorker: failed to begin insert transaction", "error", err)
			continue
		}

		var insertQuery string
		var execErr error

		if w.db.IsSQLite() {
			insertQuery = "INSERT INTO autodream_memories (id, task_id, content, embedding) VALUES (lower(hex(randomblob(16))), $1, $2, $3) ON CONFLICT(task_id) DO NOTHING"
			_, execErr = insertTx.Exec(ctx, insertQuery, task.ID, content, embStr)
		} else {
			insertQuery = "INSERT INTO autodream_memories (task_id, content, embedding) VALUES ($1, $2, $3::vector) ON CONFLICT(task_id) DO NOTHING"
			_, execErr = insertTx.Exec(ctx, insertQuery, task.ID, content, embStr)
		}

		if execErr != nil {
			insertTx.Rollback(ctx)
			slog.Error("AutoDreamWorker: failed to insert memory", "task_id", task.ID, "error", execErr)
			continue
		}

		insertTx.Commit(ctx)
		slog.Info("AutoDreamWorker: consolidated memory", "task_id", task.ID)
	}

	return nil
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
