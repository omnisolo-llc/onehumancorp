package agents

import (
	"context"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type AutoDream struct {
	db     *db.DB
	ticker *time.Ticker
	stop   chan struct{}
}

func NewAutoDream(db *db.DB) *AutoDream {
	return &AutoDream{
		db:   db,
		stop: make(chan struct{}),
	}
}

func (a *AutoDream) Start(ctx context.Context, interval time.Duration) {
	a.ticker = time.NewTicker(interval)
	go a.loop(ctx)
}

func (a *AutoDream) Stop() {
	if a.ticker != nil {
		a.ticker.Stop()
	}
	close(a.stop)
}

func (a *AutoDream) loop(ctx context.Context) {
	for {
		select {
		case <-ctx.Done():
			return
		case <-a.stop:
			return
		case <-a.ticker.C:
			a.consolidateMemories(ctx)
		}
	}
}

func (a *AutoDream) consolidateMemories(ctx context.Context) {
	slog.Info("Running autoDream memory consolidation")

	// 1. Fetch completed tasks that haven't been consolidated
	rows, err := a.db.Query(ctx, `
		SELECT id, mission_id, title, description
		FROM shared_tasks
		WHERE status = 'COMPLETED'
		-- In a real scenario, we'd add a flag or timestamp to track consolidation status
	`)
	if err != nil {
		slog.Error("Failed to query completed tasks for consolidation", "error", err)
		return
	}
	defer rows.Close()

	for rows.Next() {
		var id, missionID, title, description string
		if err := rows.Scan(&id, &missionID, &title, &description); err != nil {
			continue
		}

		// 2. Generate embedding (Mocking this for now as per instructions or using a dummy)
		// Assuming we have an embeddings client. We'll insert a mock vector or skip if not available

		content := "Task: " + title + "\nDescription: " + description

		// 3. Store in autodream_memories
		// For SQLite fallback, we just insert TEXT. For pgvector, we insert vector

		var query string
		if a.db.Provider.IsSQLite() {
			query = `INSERT INTO autodream_memories (content, embedding, source_mission_id) VALUES ($1, $2, $3)`
			// Just insert dummy text for sqlite embedding for now
			_, err = a.db.Exec(ctx, query, content, "[0.1, 0.2, 0.3]", missionID)
		} else {
			query = `INSERT INTO autodream_memories (content, embedding, source_mission_id) VALUES ($1, $2::vector, $3)`
			// PG needs array format
			_, err = a.db.Exec(ctx, query, content, "[0.1, 0.2, 0.3]", missionID)
		}

		if err != nil {
			slog.Error("Failed to insert consolidated memory", "error", err)
		}
	}
}
