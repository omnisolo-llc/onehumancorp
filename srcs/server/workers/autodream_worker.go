package workers

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type AutoDreamWorker struct {
	pool db.Provider
}

// NewAutoDreamWorker creates a new worker for autodream
func NewAutoDreamWorker(pool db.Provider) *AutoDreamWorker {
	return &AutoDreamWorker{
		pool: pool,
	}
}

func (w *AutoDreamWorker) Start(ctx context.Context) {
	slog.Info("Starting AutoDreamWorker")
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

func (w *AutoDreamWorker) ProcessCompletedTasks(ctx context.Context) {
	// Execute query inside a transaction to hold locks
	tx, err := w.pool.Begin(ctx)
	if err != nil {
		slog.Error("AutoDreamWorker: failed to begin tx", "error", err)
		return
	}
	defer tx.Rollback(ctx)

	var query string
	if w.pool.IsSQLite() {
		query = `SELECT id, organization_id, COALESCE(description, '') AS content
		          FROM shared_tasks_decomposition
		          WHERE status IN ('DONE', 'COMPLETED')
		          AND id NOT IN (SELECT source_mission_id FROM autodream_memories WHERE source_mission_id IS NOT NULL) LIMIT 10`
	} else {
		query = `SELECT id, organization_id, COALESCE(description, '') AS content
		          FROM shared_tasks_decomposition
		          WHERE status IN ('DONE', 'COMPLETED')
		          AND id NOT IN (SELECT source_mission_id FROM autodream_memories WHERE source_mission_id IS NOT NULL)
		          LIMIT 10 FOR UPDATE SKIP LOCKED`
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDreamWorker: failed to query completed tasks", "error", err)
		return
	}

	var tasks []struct {
		ID             string
		OrganizationID string
		Content        string
	}

	for rows.Next() {
		var t struct {
			ID             string
			OrganizationID string
			Content        string
		}
		if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Content); err != nil {
			slog.Error("AutoDreamWorker: failed to scan task", "error", err)
			continue
		}
		tasks = append(tasks, t)
	}
	rows.Close()
	// Lock is held until the end of tx


	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client orchestration.MinimaxClient
	if minimaxKey != "" {
		client = orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), w.pool, nil)
	}

	for _, t := range tasks {
		var embedding []float32
		if client != nil {
			embCtx, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embErr := client.GenerateEmbedding(embCtx, t.Content)
			cancel()
			if embErr == nil && len(resp) == 1536 {
				embedding = resp
			}
		}

		if len(embedding) == 0 {
			embedding = make([]float32, 1536)
		}

		strs := make([]string, len(embedding))
		for i, v := range embedding {
			strs[i] = fmt.Sprintf("%f", v)
		}
		embStr := "[" + strings.Join(strs, ",") + "]"

		memID := uuid.New().String()

		insertQuery := `INSERT INTO autodream_memories (id, organization_id, source_mission_id, content, embedding, agent_id, source_type)
		               VALUES ($1, $2, $3, $4, $5, 'autodream-worker', 'task-consolidation')`

		var execErr error
		if !w.pool.IsSQLite() {
			_, _ = tx.Exec(ctx, "SAVEPOINT autodream_insert")
		}

		if w.pool.IsSQLite() {
			_, execErr = tx.Exec(ctx, insertQuery, memID, t.OrganizationID, t.ID, t.Content, embStr)
		} else {
			insertQueryPg := `INSERT INTO autodream_memories (id, organization_id, source_mission_id, content, embedding, agent_id, source_type)
			               VALUES ($1, $2, $3, $4, $5::vector, 'autodream-worker', 'task-consolidation')`
			_, execErr = tx.Exec(ctx, insertQueryPg, memID, t.OrganizationID, t.ID, t.Content, embStr)
		}

		if execErr != nil {
			slog.Error("AutoDreamWorker: failed to insert memory", "task_id", t.ID, "error", execErr)
			if !w.pool.IsSQLite() {
				_, _ = tx.Exec(ctx, "ROLLBACK TO SAVEPOINT autodream_insert")
			}
		} else {
			if !w.pool.IsSQLite() {
				_, _ = tx.Exec(ctx, "RELEASE SAVEPOINT autodream_insert")
			}
			slog.Info("AutoDreamWorker: ingested completed task", "task_id", t.ID)
		}
	}

	tx.Commit(ctx)
}
