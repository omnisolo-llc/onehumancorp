package agents

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// AutoDreamEngine consolidates swarm memory and tasks into long-term vector embeddings.
type AutoDreamEngine struct {
	db          db.Provider
	minimax     *orchestration.MinimaxClient
	pollInterval time.Duration
}

// NewAutoDreamEngine creates a new AutoDreamEngine.
func NewAutoDreamEngine(db db.Provider, apiKey string) *AutoDreamEngine {
	return &AutoDreamEngine{
		db:           db,
		minimax:      orchestration.NewMinimaxClient(apiKey),
		pollInterval: 1 * time.Hour,
	}
}

// Start begins the autoDream daemon loop.
func (e *AutoDreamEngine) Start(ctx context.Context) {
	ticker := time.NewTicker(e.pollInterval)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				e.consolidateMemories(ctx)
			}
		}
	}()
}

// consolidateMemories sweeps completed swarm_tasks and swarm_memory to consolidate into swarm_long_term_memory
func (e *AutoDreamEngine) consolidateMemories(ctx context.Context) {
	if e.minimax.APIKey == "" {
		return // Cannot consolidate without LLM
	}

	// 1. Fetch recently completed tasks to consolidate
	rows, err := e.db.Query(ctx, "SELECT id, title, payload FROM swarm_tasks WHERE status = 'COMPLETED' AND updated_at > $1 LIMIT 50", time.Now().Add(-24*time.Hour))
	if err != nil {
		if err.Error() != "sql: no rows in result set" {
			slog.Error("autodream: failed to query completed tasks", "err", err)
		}
		// Try SQLite specific query fallback if error contains parameter issue (e.g. $1)
		rows, err = e.db.Query(ctx, "SELECT id, title, payload FROM swarm_tasks WHERE status = 'COMPLETED' AND updated_at > ? LIMIT 50", time.Now().Add(-24*time.Hour))
		if err != nil {
			return
		}
	}
	defer rows.Close()

	var tasks []struct{ ID, Title, Payload string }
	for rows.Next() {
		var t struct{ ID, Title, Payload string }
		if err := rows.Scan(&t.ID, &t.Title, &t.Payload); err == nil {
			tasks = append(tasks, t)
		}
	}

	if len(tasks) == 0 {
		return
	}

	// For each task, summarize and store
	for _, task := range tasks {
		prompt := fmt.Sprintf("Consolidate the following completed swarm task into a durable architectural memory summary.\nTitle: %s\nPayload: %s\nOutput a concise summary:", task.Title, task.Payload)
		summary, err := e.minimax.Reason(ctx, prompt)
		if err != nil {
			slog.Warn("autodream: minimax reasoning failed", "task_id", task.ID, "err", err)
			continue
		}

		// Mock embedding generation for now, ideally we use Minimax Embedding API
		// We'll store an empty or mock 1536 dim embedding vector.
		mockEmbedding := make([]float32, 1536)
		mockEmbedding[0] = 0.5 // Just to have non-zero data

		embeddingBytes, _ := json.Marshal(mockEmbedding)

		if e.db.IsSQLite() {
			_, err = e.db.Exec(ctx,
				"INSERT INTO swarm_long_term_memory (topic, summary, embedding, created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)",
				task.Title, summary, embeddingBytes)
		} else {
			_, err = e.db.Exec(ctx,
				"INSERT INTO swarm_long_term_memory (topic, summary, embedding, created_at) VALUES ($1, $2, $3, CURRENT_TIMESTAMP)",
				task.Title, summary, embeddingBytes)
		}

		if err != nil {
			slog.Error("autodream: failed to store memory", "task_id", task.ID, "err", err)
		} else {
			slog.Info("autodream: consolidated task into memory", "task_id", task.ID)
		}
	}
}
