package agents

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type AutoDreamEngine struct {
	db          db.Provider
	llmProvider *LLMProvider
}

func NewAutoDreamEngine(db db.Provider, provider *LLMProvider) *AutoDreamEngine {
	return &AutoDreamEngine{
		db:          db,
		llmProvider: provider,
	}
}

func (e *AutoDreamEngine) Start(ctx context.Context) {
	ticker := time.NewTicker(15 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			e.ConsolidateMemories(ctx)
		}
	}
}

func (e *AutoDreamEngine) ConsolidateMemories(ctx context.Context) {
	slog.Info("autodream: starting memory consolidation")

	// In a real implementation, we would query `shared_tasks` where status = 'COMPLETED'
	// and extract meaningful context, generating embeddings and storing in `autodream_memories`.
	// For now, we simulate this process to demonstrate hybrid persistence.

	tasks, err := e.getCompletedTasks(ctx)
	if err != nil {
		slog.Error("autodream: failed to fetch completed tasks", "error", err)
		return
	}

	for _, task := range tasks {
		// Mock generated embedding based on task description
		embedding := e.mockGenerateEmbedding(task.Description)

		err = e.storeMemory(ctx, task.MissionID, task.Title+" - "+task.Description, embedding)
		if err != nil {
			slog.Error("autodream: failed to store memory", "error", err, "mission_id", task.MissionID)
		} else {
			slog.Info("autodream: memory consolidated", "mission_id", task.MissionID)
		}

		// Mark task as consolidated (we could add a new status or column, here we just log)
	}
}

func (e *AutoDreamEngine) getCompletedTasks(ctx context.Context) ([]orchestration.SharedTask, error) {
	// Let's get completed tasks
	rows, err := e.db.Query(ctx, "SELECT id, mission_id, title, description FROM shared_tasks WHERE status = 'COMPLETED'")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []orchestration.SharedTask
	for rows.Next() {
		var t orchestration.SharedTask
		if err := rows.Scan(&t.ID, &t.MissionID, &t.Title, &t.Description); err != nil {
			return nil, err
		}
		tasks = append(tasks, t)
	}
	return tasks, nil
}

func (e *AutoDreamEngine) mockGenerateEmbedding(text string) []float32 {
	// Return a dummy 1536-dimensional vector
	vec := make([]float32, 1536)
	for i := range vec {
		vec[i] = 0.1 // Dummy data
	}
	return vec
}

func (e *AutoDreamEngine) storeMemory(ctx context.Context, missionID, content string, embedding []float32) error {
	// We handle vector differently between Postgres (pgvector) and SQLite (blob)
	if e.db.IsSQLite() {
		// SQLite: store as BLOB (dummy byte representation for now)
		blob := make([]byte, len(embedding)*4) // simplified encoding
		_, err := e.db.Exec(ctx,
			"INSERT INTO autodream_memories (content, embedding, source_mission_id) VALUES (?, ?, ?)",
			content, blob, missionID)
		return err
	}

	// Postgres: string representation of vector
	vecStr := "["
	for i, v := range embedding {
		vecStr += fmt.Sprintf("%f", v)
		if i < len(embedding)-1 {
			vecStr += ","
		}
	}
	vecStr += "]"

	_, err := e.db.Exec(ctx,
		"INSERT INTO autodream_memories (content, embedding, source_mission_id) VALUES ($1, $2, $3)",
		content, vecStr, missionID)
	return err
}
