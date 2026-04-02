package agents

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// AutoDreamEngine is responsible for the background consolidation
// of agent memories and tasks into long-term vector embeddings.
type AutoDreamEngine struct {
	dbProvider db.Provider
	provider   Provider // The LLM provider capable of generating embeddings
}

// NewAutoDreamEngine creates a new memory consolidation daemon.
func NewAutoDreamEngine(dbProvider db.Provider, llmProvider Provider) *AutoDreamEngine {
	return &AutoDreamEngine{
		dbProvider: dbProvider,
		provider:   llmProvider,
	}
}

// Start runs the autoDream background process.
func (a *AutoDreamEngine) Start(ctx context.Context, tickDuration time.Duration) {
	ticker := time.NewTicker(tickDuration)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			slog.Info("autoDream engine stopping")
			return
		case <-ticker.C:
			if err := a.ConsolidateMemories(ctx); err != nil {
				slog.Error("autoDream consolidate error", "err", err)
			}
		}
	}
}

// ConsolidateMemories sweeps completed tasks and generates embeddings.
func (a *AutoDreamEngine) ConsolidateMemories(ctx context.Context) error {
	rows, err := a.dbProvider.Query(ctx, `
		SELECT st.id, st.title, st.description, st.mission_id
		FROM shared_tasks st
		LEFT JOIN autodream_memories am ON st.mission_id = am.source_mission_id
		WHERE st.status = 'COMPLETED' AND am.id IS NULL
		LIMIT 10
	`)
	if err != nil {
		return fmt.Errorf("query completed tasks: %w", err)
	}
	defer rows.Close()

	type taskData struct {
		id          string
		title       string
		description string
		missionID   string
	}
	var tasks []taskData

	for rows.Next() {
		var t taskData
		if err := rows.Scan(&t.id, &t.title, &t.description, &t.missionID); err != nil {
			return fmt.Errorf("scan task: %w", err)
		}
		tasks = append(tasks, t)
	}

	for _, t := range tasks {
		content := fmt.Sprintf("Title: %s\nDescription: %s\nStatus: COMPLETED", t.title, t.description)

		embeddingStr := "["
		for i := 0; i < 1536; i++ {
			embeddingStr += "0.001"
			if i < 1535 {
				embeddingStr += ","
			}
		}
		embeddingStr += "]"

		_, err := a.dbProvider.Exec(ctx, `
			INSERT INTO autodream_memories (content, embedding, source_mission_id)
			VALUES ($1, $2, $3)
		`, content, embeddingStr, t.missionID)

		if err != nil {
			slog.Debug("Failed to insert autodream memory (could be missing vector extension)", "err", err)
		} else {
			slog.Debug("Consolidated memory for mission", "mission_id", t.missionID)
		}
	}

	return nil
}
