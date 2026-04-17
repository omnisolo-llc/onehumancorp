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
	query := `SELECT id, organization_id, COALESCE(description, '') AS content
	          FROM shared_tasks_decomposition
	          WHERE status IN ('DONE', 'COMPLETED')
	          AND id NOT IN (SELECT source_mission_id FROM autodream_memories WHERE source_mission_id IS NOT NULL)`

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

		memID := uuid.New().String()

		insertQuery := `INSERT INTO autodream_memories (id, organization_id, source_mission_id, content, embedding, agent_id, source_type)
		               VALUES ($1, $2, $3, $4, $5, 'autodream-worker', 'task-consolidation')`
		_, err := w.pool.Exec(ctx, insertQuery, memID, t.OrganizationID, t.ID, t.Content, embStr)
		if err != nil {
			insertQueryPg := `INSERT INTO autodream_memories (id, organization_id, source_mission_id, content, embedding, agent_id, source_type)
			               VALUES ($1, $2, $3, $4, $5::vector, 'autodream-worker', 'task-consolidation')`
			_, errPg := w.pool.Exec(ctx, insertQueryPg, memID, t.OrganizationID, t.ID, t.Content, embStr)
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
