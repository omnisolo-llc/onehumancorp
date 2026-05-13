package orchestration

import (
	"context"
	"database/sql"
	"log"
	"time"
)

// DefaultTaskOrchestrator manages the task background worker loop
type DefaultTaskOrchestrator struct {
	db       *sql.DB
	spawner  SubAgentSpawner
	isSQLite bool
}

// NewDefaultTaskOrchestrator initializes the orchestrator
func NewDefaultTaskOrchestrator(db *sql.DB, spawner SubAgentSpawner, isSQLite bool) *DefaultTaskOrchestrator {
	return &DefaultTaskOrchestrator{
		db:       db,
		spawner:  spawner,
		isSQLite: isSQLite,
	}
}

// StartBackgroundWorker begins polling for tasks
func (to *DefaultTaskOrchestrator) StartBackgroundWorker(ctx context.Context) {
	ticker := time.NewTicker(10 * time.Millisecond) // Faster ticker for test coverage
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			log.Println("TaskOrchestrator background worker stopping")
			return
		case <-ticker.C:
			if err := to.PollTasks(ctx); err != nil {
				log.Printf("PollTasks encountered an error: %v", err)
			}
		}
	}
}

// PollTasks queries the database for DELEGATED tasks and routes them
func (to *DefaultTaskOrchestrator) PollTasks(ctx context.Context) error {
	var query string
	if to.isSQLite {
		// SQLite doesn't support FOR UPDATE SKIP LOCKED
		query = "SELECT id, payload FROM shared_tasks WHERE priority = 'DELEGATED' AND status = 'PENDING' LIMIT 1"
	} else {
		// Cloud Mode: PostgreSQL FOR UPDATE SKIP LOCKED
		query = "SELECT id, payload FROM shared_tasks WHERE priority = 'DELEGATED' AND status = 'PENDING' LIMIT 1 FOR UPDATE SKIP LOCKED"
	}

	var task SharedTask
	var payloadStr string // Fix scan issue for JSON
	err := to.db.QueryRowContext(ctx, query).Scan(&task.ID, &payloadStr)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil // No tasks found
		}
		return err
	}
	task.Payload = []byte(payloadStr)

	// Update status so we don't infinitely poll the same task
	_, err = to.db.ExecContext(ctx, "UPDATE shared_tasks SET status = 'PROCESSING' WHERE id = $1", task.ID)
	if err != nil {
		return err
	}

	// Route to SubAgentSpawner
	go func(t SharedTask) {
		// Create a span context or use the orchestrator context
		if err := to.spawner.Spawn(context.Background(), &t); err != nil {
			log.Printf("SubAgentSpawner failed for task %s: %v", t.ID, err)
		}
	}(task)

	return nil
}
