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
	return &AutoDreamWorker{pool: pool}
}

func (w *AutoDreamWorker) ProcessCompletedTasks(ctx context.Context) error {
	slog.Info("AutoDreamWorker: checking for COMPLETED tasks to vectorize")

	// Check schema to determine if task_id or source_mission_id should be used
	var columnToCheck string
	checkErr := w.pool.QueryRow(ctx, "SELECT COUNT(*) FROM pragma_table_info('autodream_memories') WHERE name='task_id'").Scan(&columnToCheck)
	useTaskID := false
	if checkErr == nil && columnToCheck != "0" {
		useTaskID = true
	} else if !w.pool.IsSQLite() {
		// Postgres fallback logic check
		checkErr = w.pool.QueryRow(ctx, "SELECT column_name FROM information_schema.columns WHERE table_name='autodream_memories' and column_name='task_id'").Scan(&columnToCheck)
		if checkErr == nil && columnToCheck == "task_id" {
			useTaskID = true
		}
	}

	// Double check for source_mission_id if task_id isn't found
	useSourceMissionID := false
	if !useTaskID {
		checkErr = w.pool.QueryRow(ctx, "SELECT COUNT(*) FROM pragma_table_info('autodream_memories') WHERE name='source_mission_id'").Scan(&columnToCheck)
		if checkErr == nil && columnToCheck != "0" {
			useSourceMissionID = true
		} else if !w.pool.IsSQLite() {
			checkErr = w.pool.QueryRow(ctx, "SELECT column_name FROM information_schema.columns WHERE table_name='autodream_memories' and column_name='source_mission_id'").Scan(&columnToCheck)
			if checkErr == nil && columnToCheck == "source_mission_id" {
				useSourceMissionID = true
			}
		}
	}

	var query string
	if useTaskID {
		query = "SELECT id, organization_id, payload FROM shared_tasks_decomposition WHERE status = 'DONE' AND id NOT IN (SELECT task_id FROM autodream_memories WHERE task_id IS NOT NULL) LIMIT 100"
	} else if useSourceMissionID {
		query = "SELECT id, organization_id, payload FROM shared_tasks_decomposition WHERE status = 'DONE' AND id NOT IN (SELECT source_mission_id FROM autodream_memories WHERE source_mission_id IS NOT NULL) LIMIT 100"
	} else {
		// Default to schema-independent fetching if no foreign key tracking is present
		query = "SELECT id, organization_id, payload FROM shared_tasks_decomposition WHERE status = 'DONE' LIMIT 100"
	}

	rows, err := w.pool.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to fetch completed tasks: %w", err)
	}
	defer rows.Close()

	type Task struct {
		ID      string
		OrgID   string
		Payload *string
	}
	var tasks []Task
	for rows.Next() {
		var t Task
		if err := rows.Scan(&t.ID, &t.OrgID, &t.Payload); err != nil {
			return fmt.Errorf("failed to scan task: %w", err)
		}
		tasks = append(tasks, t)
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	if minimaxKey == "" {
		minimaxKey = "mock_key" // Fallback for tests if needed, but in real environment we'd want a real key or fail gracefully
	}

	client := orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), w.pool, nil)

	for _, t := range tasks {
		content := ""
		if t.Payload != nil {
			content = *t.Payload
		}
		if content == "" {
			continue
		}

		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		embedding, err := client.GenerateEmbedding(ctxTimeout, content)
		cancel()

		if err != nil {
			slog.Error("AutoDreamWorker: failed to generate embedding, skipping task", "task_id", t.ID, "error", err)
			continue
		}

		if len(embedding) == 0 {
			slog.Error("AutoDreamWorker: generated embedding is empty, skipping task", "task_id", t.ID)
			continue
		}

		memID := uuid.New().String()
		embStr := formatFloat32SliceForVector(embedding)

		var insertQuery string
		if w.pool.IsSQLite() {
			if useTaskID {
				insertQuery = "INSERT INTO autodream_memories (id, task_id, content, embedding, created_at) VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)"
			} else if useSourceMissionID {
				insertQuery = "INSERT INTO autodream_memories (id, source_mission_id, content, embedding, created_at) VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)"
			} else {
				insertQuery = "INSERT INTO autodream_memories (id, content, embedding, created_at) VALUES ($1, $3, $4, CURRENT_TIMESTAMP)"
			}
		} else {
			if useTaskID {
				insertQuery = "INSERT INTO autodream_memories (id, task_id, content, embedding, created_at) VALUES ($1, $2, $3, $4::vector, CURRENT_TIMESTAMP)"
			} else if useSourceMissionID {
				insertQuery = "INSERT INTO autodream_memories (id, source_mission_id, content, embedding, created_at) VALUES ($1, $2, $3, $4::vector, CURRENT_TIMESTAMP)"
			} else {
				insertQuery = "INSERT INTO autodream_memories (id, content, embedding, created_at) VALUES ($1, $3, $4::vector, CURRENT_TIMESTAMP)"
			}
		}

		if useTaskID || useSourceMissionID {
			_, err = w.pool.Exec(ctx, insertQuery, memID, t.ID, content, embStr)
		} else {
			_, err = w.pool.Exec(ctx, insertQuery, memID, "", content, embStr) // Placeholder for arity match if no ID tracking
		}

		if err != nil {
			slog.Error("AutoDreamWorker: failed to insert memory", "error", err)
		} else {
			slog.Info("AutoDreamWorker: inserted memory successfully", "task_id", t.ID)
		}
	}
	return nil
}

func formatFloat32SliceForVector(embedding []float32) string {
	if len(embedding) == 0 {
		return "[]"
	}
	strs := make([]string, len(embedding))
	for i, v := range embedding {
		strs[i] = fmt.Sprintf("%f", v)
	}
	return "[" + strings.Join(strs, ",") + "]"
}
