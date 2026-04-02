package agents

import (
	"context"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// AutoDreamEngine consolidates completed tasks into long-term swarm memory.
type AutoDreamEngine struct {
	pool db.Provider
}

// NewAutoDreamEngine creates a new background engine for memory consolidation.
func NewAutoDreamEngine(pool db.Provider) *AutoDreamEngine {
	return &AutoDreamEngine{pool: pool}
}

// Start spawns the background worker for autoDream.
func (e *AutoDreamEngine) Start(ctx context.Context, tickDuration time.Duration) {
	ticker := time.NewTicker(tickDuration)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			e.ProcessTick(ctx)
		}
	}
}

// ProcessTick executes a single autoDream consolidation loop.
func (e *AutoDreamEngine) ProcessTick(ctx context.Context) {
	slog.Debug("autoDream: running memory consolidation sweep")

	// Real implementation would sweep completed `swarm_tasks` and generate embeddings
	// using an LLM, then store them in `swarm_long_term_memory`.
	// For now, we perform a basic query to ensure it doesn't break.

	if e.pool == nil {
		return
	}

	_, err := e.pool.Exec(ctx, `
		INSERT INTO swarm_long_term_memory (topic, summary)
		SELECT title, 'auto-consolidated from completed task'
		FROM swarm_tasks
		WHERE status = 'COMPLETED'
		ON CONFLICT DO NOTHING
	`)

	if err != nil {
		// Log silently as background loop
		_ = err
	}
}
