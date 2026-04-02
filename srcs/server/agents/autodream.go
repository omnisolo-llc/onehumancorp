package agents

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// StartAutoDreamEngine starts a background daemon that periodically sweeps
// completed shared tasks and consolidates them into long-term vector memories.
// This is critical for the Swarm to maintain architectural insights across sessions.
func StartAutoDreamEngine(ctx context.Context, hub *orchestration.Hub, llmClient *orchestration.MinimaxClient) {
	// Expose ticker control for testing, here we just use 6 hours
	ticker := time.NewTicker(6 * time.Hour)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			processAutoDreamCycle(ctx, hub, llmClient)
		}
	}
}

// processAutoDreamCycle sweeps the DB for COMPLETED tasks and pushes them into long-term memory.
func processAutoDreamCycle(ctx context.Context, hub *orchestration.Hub, llmClient *orchestration.MinimaxClient) {
	sipDB := hub.GetSIPDB()
	if sipDB == nil {
		slog.Warn("[autoDream] SIPDB not initialized, skipping cycle")
		return
	}

	// Fetch recent completed tasks
	tasks, err := hub.GetCompletedTasksForAutoDream(ctx, 10)
	if err != nil {
		slog.Error("[autoDream] Failed to fetch completed tasks", "error", err)
		return
	}

	if len(tasks) == 0 {
		return
	}

	// Consolidate
	for _, task := range tasks {
		payloadBytes, _ := json.Marshal(task)
		prompt := fmt.Sprintf("Analyze this completed swarm task. Extract the core architectural decisions, lessons learned, and systemic context into a dense summary. Task Data:\n%s", string(payloadBytes))

		// In a real scenario, we might use a dedicated Embedding API, but here we use the Minimax reasoning API
		// to generate a text summary, and we'll mock the vector embedding generation.

		summary, err := llmClient.Reason(ctx, prompt)
		if err != nil {
			slog.Warn("[autoDream] Failed to generate summary for task", "task_id", task.ID, "error", err)
			continue
		}

		// Mock embedding generation (e.g., using pgvector <-> []float32)
		// For simplicity, we just store a nil byte array here, but in production we'd call an embedding model.
		var embedding []byte

		err = hub.StoreAutoDreamMemory(ctx, task.Title, summary, embedding)
		if err != nil {
			slog.Error("[autoDream] Failed to store memory", "task_id", task.ID, "error", err)
		} else {
			slog.Info("[autoDream] Memory consolidated successfully", "task_id", task.ID)
		}
	}
}
