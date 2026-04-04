package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"time"
)

// dummyVector generates a 1536-dimensional zero vector string for fallback.
func dummyVector() string {
	b := make([]byte, 0, 1536*3+2)
	b = append(b, '[')
	for i := 0; i < 1536; i++ {
		if i > 0 {
			b = append(b, ',', ' ')
		}
		b = append(b, '0')
	}
	b = append(b, ']')
	return string(b)
}

// ProcessCompletedTasks checks for COMPLETED shared_tasks and vectors their payload.
func (w *AutoDreamWorker) ProcessCompletedTasks(ctx context.Context) error {
	slog.Info("AutoDream: Sweeping COMPLETED tasks for memory consolidation")

	tx, err := w.pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if w.pool.IsSQLite() {
		query = "SELECT id, mission_id, title, payload FROM swarm_tasks WHERE status = 'COMPLETED' LIMIT 10"
	} else {
		query = "SELECT id, mission_id, title, payload FROM swarm_tasks WHERE status = 'COMPLETED' LIMIT 10 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query completed tasks: %w", err)
	}

	var tasksToProcess []struct {
		ID        string
		MissionID string
		Title     string
		Payload   string
	}
	for rows.Next() {
		var t struct {
			ID        string
			MissionID string
			Title     string
			Payload   string
		}
		if err := rows.Scan(&t.ID, &t.MissionID, &t.Title, &t.Payload); err == nil {
			tasksToProcess = append(tasksToProcess, t)
		}
	}
	rows.Close()

	if len(tasksToProcess) == 0 {
		return nil
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client MinimaxClient
	if minimaxKey != "" {
		client = NewMinimaxClient(minimaxKey)
	}

	for _, task := range tasksToProcess {
		slog.Info("AutoDream: embedding payload for task", "task_id", task.ID)

		content := fmt.Sprintf("Mission: %s\nTitle: %s\nPayload: %s", task.MissionID, task.Title, task.Payload)

		var embeddingStr string
		if client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			vec, err := client.GenerateEmbedding(ctxTimeout, content)
			cancel()
			if err == nil && len(vec) > 0 {
				embeddingBytes, _ := json.Marshal(vec)
				embeddingStr = string(embeddingBytes)
			} else {
				slog.Warn("AutoDream: embedding failed", "error", err)
				embeddingStr = dummyVector()
			}
		} else {
			embeddingStr = dummyVector() // dummy vector
		}

		insertQuery := "INSERT INTO autodream_memories (content, embedding, source_mission_id, consolidated_at) VALUES ($1, $2::vector, $3, NOW())"
		if w.pool.IsSQLite() {
			insertQuery = "INSERT INTO autodream_memories (content, embedding, source_mission_id, consolidated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)"
		}

		_, err = tx.Exec(ctx, insertQuery, content, embeddingStr, task.MissionID)
		if err != nil {
			slog.Error("AutoDream: failed to insert autodream_memory", "error", err)
			// Explicitly fail/archive the task to avoid infinite loops if insertion fails (e.g., due to pgvector dim mismatch)
			failQuery := "UPDATE swarm_tasks SET status = 'FAILED' WHERE id = $1"
			if w.pool.IsSQLite() {
				failQuery = "UPDATE swarm_tasks SET status = 'FAILED' WHERE id = ?"
			}
			_, _ = tx.Exec(ctx, failQuery, task.ID)
			continue
		}

		deleteQuery := "DELETE FROM swarm_tasks WHERE id = $1"
		if w.pool.IsSQLite() {
			deleteQuery = "DELETE FROM swarm_tasks WHERE id = ?"
		}
		_, _ = tx.Exec(ctx, deleteQuery, task.ID)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit AutoDream task process: %w", err)
	}

	return nil
}
