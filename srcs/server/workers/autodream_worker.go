package workers

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type AutoDreamWorker struct {
	pool db.Provider
}

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
	// The problem states we need to implement AutoDreamWorker.
	// Since consolidated_memory does not have source_mission_id, we will format content as "Task [ID]: [content]"
	query := `SELECT id, organization_id, COALESCE(description, '') AS content
	          FROM shared_tasks_decomposition
	          WHERE status IN ('DONE', 'COMPLETED')
	          AND id NOT IN (SELECT id FROM consolidated_memory WHERE source_type = 'task-consolidation')`

	rows, err := w.pool.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDreamWorker: failed to query completed tasks", "error", err)
		return
	}
	defer rows.Close()

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

		// We will use t.ID as the memID so we can avoid duplicates easily (since we check for `id NOT IN ... WHERE source_type = 'task-consolidation'`)
		// But in the query above we did `id NOT IN (SELECT id FROM consolidated_memory)`. Wait, we should use task ID as the consolidated_memory ID.
		memID := t.ID
		content := fmt.Sprintf("Task [%s]: %s", t.ID, t.Content)

		insertQuery := `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type)
		               VALUES ($1, $2, 'autodream-worker', $3, $4, 'task-consolidation')`
		_, err := w.pool.Exec(ctx, insertQuery, memID, t.OrganizationID, content, embStr)
		if err != nil {
			insertQueryPg := `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type)
			               VALUES ($1, $2, 'autodream-worker', $3, $4::vector, 'task-consolidation')`
			_, errPg := w.pool.Exec(ctx, insertQueryPg, memID, t.OrganizationID, content, embStr)
			if errPg != nil {
				slog.Error("AutoDreamWorker: failed to insert memory", "task_id", t.ID, "error", errPg)
			} else {
				slog.Info("AutoDreamWorker: ingested completed task (pg fallback)", "task_id", t.ID)
			}
		} else {
			slog.Info("AutoDreamWorker: ingested completed task", "task_id", t.ID)
		}
	}
}
