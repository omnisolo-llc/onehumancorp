package workers

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type AutoDreamWorker struct {
	pool db.Provider
}

func NewAutoDreamWorker(pool db.Provider) *AutoDreamWorker {
	return &AutoDreamWorker{pool: pool}
}

func (w *AutoDreamWorker) Start(ctx context.Context) {
	slog.Info("Starting AutoDreamWorker memory consolidation")
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.consolidateMemories(ctx)
		}
	}
}

func (w *AutoDreamWorker) consolidateMemories(ctx context.Context) {
	tx, err := w.pool.Begin(ctx)
	if err != nil {
		slog.Error("AutoDreamWorker: failed to begin transaction", "error", err)
		return
	}
	defer tx.Rollback(ctx)

	var query string
	if w.pool.IsSQLite() {
		query = "SELECT id, title, description, payload, organization_id FROM shared_tasks_decomposition WHERE status = 'COMPLETED' LIMIT 50"
	} else {
		query = "SELECT id, title, description, payload, organization_id FROM shared_tasks_decomposition WHERE status = 'COMPLETED' LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDreamWorker: failed to query completed tasks", "error", err)
		return
	}

	type task struct {
		id          string
		title       string
		description string
		payload     string
		orgID       string
	}

	var tasks []task
	for rows.Next() {
		var t task
		var payload *string
		var desc *string
		if err := rows.Scan(&t.id, &t.title, &desc, &payload, &t.orgID); err != nil {
			continue
		}
		if desc != nil {
			t.description = *desc
		}
		if payload != nil {
			t.payload = *payload
		}
		tasks = append(tasks, t)
	}
	rows.Close()

	if len(tasks) == 0 {
		return
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client orchestration.MinimaxClient
	if minimaxKey != "" {
		client = orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), w.pool, nil)
	}

	for _, t := range tasks {
		content := fmt.Sprintf("Title: %s\nDescription: %s\nPayload: %s", t.title, t.description, t.payload)

		embedding := make([]float32, 1536)
		if client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := client.GenerateEmbedding(ctxTimeout, content)
			cancel()
			if err == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Warn("AutoDreamWorker: failed to embed with Minimax, using empty embedding", "error", err)
			}
		}

		strs := make([]string, len(embedding))
		for i, v := range embedding {
			strs[i] = fmt.Sprintf("%f", v)
		}
		embStr := "["
		for i, s := range strs {
			if i > 0 {
				embStr += ","
			}
			embStr += s
		}
		embStr += "]"

		memID := uuid.New().String()

		var insertQuery string
		if w.pool.IsSQLite() {
			insertQuery = `INSERT INTO autodream_memories (id, task_id, organization_id, content, embedding, created_at) VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)`
		} else {
			insertQuery = `INSERT INTO autodream_memories (id, task_id, organization_id, content, embedding, created_at) VALUES ($1, $2, $3, $4, $5::vector, CURRENT_TIMESTAMP)`
		}

		_, err = tx.Exec(ctx, insertQuery, memID, t.id, t.orgID, content, embStr)
		if err != nil {
			slog.Error("AutoDreamWorker: failed to insert memory", "error", err)
			continue
		}

		_, err = tx.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'CONSOLIDATED' WHERE id = $1", t.id)
		if err != nil {
			slog.Error("AutoDreamWorker: failed to update task status", "error", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		slog.Error("AutoDreamWorker: failed to commit transaction", "error", err)
	}
}
