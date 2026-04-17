package orchestration

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// AutoDreamTaskWorker periodically scans for completed tasks to vectorize them
type AutoDreamTaskWorker struct {
	db       db.Provider
	interval time.Duration
	stopCh   chan struct{}
}

// NewAutoDreamTaskWorker creates a new worker
func NewAutoDreamTaskWorker(db db.Provider, interval time.Duration) *AutoDreamTaskWorker {
	if interval == 0 {
		interval = 1 * time.Minute
	}
	return &AutoDreamTaskWorker{
		db:       db,
		interval: interval,
		stopCh:   make(chan struct{}),
	}
}

// Start begins the background worker loop
func (w *AutoDreamTaskWorker) Start() {
	go w.loop()
}

// Stop halts the background worker
func (w *AutoDreamTaskWorker) Stop() {
	close(w.stopCh)
}

func (w *AutoDreamTaskWorker) loop() {
	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	for {
		select {
		case <-w.stopCh:
			return
		case <-ticker.C:
			w.processCompletedTasks(context.Background())
		}
	}
}

func (w *AutoDreamTaskWorker) processCompletedTasks(ctx context.Context) {
	// Find completed tasks that haven't been vectorized yet.
	query := `
		SELECT st.id, st.title, st.payload
		FROM shared_tasks st
		LEFT JOIN autodream_memories am ON st.id = am.task_id
		WHERE st.status = 'COMPLETED' AND am.id IS NULL
		LIMIT 10
	`
	rows, err := w.db.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDreamTaskWorker: failed to query completed tasks", "error", err)
		return
	}
	defer rows.Close()

	for rows.Next() {
		var taskID, title string
		var payloadStr string

		if err := rows.Scan(&taskID, &title, &payloadStr); err != nil {
			slog.Error("AutoDreamTaskWorker: failed to scan task", "error", err)
			continue
		}

		// Stub out vector embedding logic
		content := fmt.Sprintf("Task: %s\nPayload: %s", title, payloadStr)

		if !w.db.IsSQLite() {
			// PG supports pgvector, insert a zero vector or similar for testing
			// Assuming vector(1536)
			insertQuery := `
				INSERT INTO autodream_memories (task_id, content, embedding)
				VALUES ($1, $2, array_fill(0, ARRAY[1536])::vector)
			`
			_, err = w.db.Exec(ctx, insertQuery, taskID, content)
			if err != nil {
				slog.Error("AutoDreamTaskWorker: failed to insert memory", "task_id", taskID, "error", err)
			} else {
				slog.Info("AutoDreamTaskWorker: vectorized task", "task_id", taskID)
			}
		} else {
			// SQLite fallback
			insertQuery := `
				INSERT INTO autodream_memories (task_id, content)
				VALUES ($1, $2)
			`
			_, err = w.db.Exec(ctx, insertQuery, taskID, content)
			if err != nil {
				slog.Error("AutoDreamTaskWorker: failed to insert memory (sqlite)", "task_id", taskID, "error", err)
			} else {
				slog.Info("AutoDreamTaskWorker: vectorized task (sqlite)", "task_id", taskID)
			}
		}
	}
}
