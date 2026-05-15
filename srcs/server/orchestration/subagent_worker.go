package orchestration

import (
	"context"
	"database/sql"
	"log"
	"sync"
	"time"

	"go.opentelemetry.io/otel"
)

type SubAgentWorker struct {
	db       *sql.DB
	sm       *TaskStateMachine
	spawner  SubAgentSpawner
	isSQLite bool
	mu       sync.Mutex
}

func NewSubAgentWorker(db *sql.DB, sm *TaskStateMachine, spawner SubAgentSpawner) *SubAgentWorker {
	var isSqlite bool
	err := db.QueryRow("SELECT sqlite_version()").Scan(new(string))
	isSqlite = err == nil

	return &SubAgentWorker{
		db:       db,
		sm:       sm,
		spawner:  spawner,
		isSQLite: isSqlite,
	}
}

func (w *SubAgentWorker) Start(ctx context.Context) {
	go func() {
		ticker := time.NewTicker(5 * time.Second)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				w.Poll(ctx)
			}
		}
	}()
}

func (w *SubAgentWorker) Poll(ctx context.Context) {
	for {
		if w.isSQLite {
			w.mu.Lock()
		}

		tx, err := w.db.BeginTx(ctx, nil)
		if err != nil {
			if w.isSQLite {
				w.mu.Unlock()
			}
			log.Printf("Failed to begin tx: %v", err)
			return
		}

		// Find a pending job
		var id, taskID string
		query := "SELECT id, task_id FROM sub_agent_jobs WHERE status = 'PENDING' LIMIT 1"
		if !w.isSQLite {
			query += " FOR UPDATE SKIP LOCKED"
		}

		err = tx.QueryRowContext(ctx, query).Scan(&id, &taskID)
		if err != nil {
			tx.Rollback()
			if w.isSQLite {
				w.mu.Unlock()
			}
			if err != sql.ErrNoRows {
				log.Printf("Failed to poll jobs: %v", err)
			}
			return
		}

		// Update to RUNNING
		_, err = tx.ExecContext(ctx, "UPDATE sub_agent_jobs SET status = 'RUNNING', updated_at = CURRENT_TIMESTAMP WHERE id = $1", id)
		if err != nil {
			tx.Rollback()
			if w.isSQLite {
				w.mu.Unlock()
			}
			log.Printf("Failed to update job status: %v", err)
			return
		}

		if err := tx.Commit(); err != nil {
			if w.isSQLite {
				w.mu.Unlock()
			}
			log.Printf("Failed to commit job claim: %v", err)
			return
		}

		if w.isSQLite {
			w.mu.Unlock()
		}

		// Process the job
		go w.processJob(context.Background(), id, taskID)
	}
}

func (w *SubAgentWorker) processJob(ctx context.Context, jobID, taskID string) {
	start := time.Now()
	// Create a dummy SharedTask for the spawner

	job := &Job{ID: jobID, TaskID: taskID}

	// Inform state machine that job is running
	_ = w.sm.ProcessEvent(ctx, taskID, EventDecompositionComplete)

	err := w.spawner.SpawnIsolated(ctx, job)

	duration := time.Since(start).Seconds()
	meter := otel.Meter("subagent_worker")
	execDuration, _ := meter.Float64Histogram("ohc_sub_agent_execution_duration_seconds")
	execDuration.Record(ctx, duration)

	// Update job status
	var status string
	var event TaskEvent

	if err != nil {
		status = "FAILED"
		event = EventSubTaskFailed
		failures, _ := meter.Int64Counter("ohc_sub_agent_failures_total")
		failures.Add(ctx, 1)
	} else {
		status = "COMPLETED"
		event = EventSubTaskCompleted
	}

	// Lock if SQLite to update DB
	if w.isSQLite {
		w.mu.Lock()
		defer w.mu.Unlock()
	}

	tx, txErr := w.db.BeginTx(ctx, nil)
	if txErr == nil {
		_, execErr := tx.ExecContext(ctx, "UPDATE sub_agent_jobs SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2", status, jobID)
		if execErr == nil {
			_ = tx.Commit()
		} else {
			_ = tx.Rollback()
			return
		}
	} else {
		return
	}

	// Inform state machine
	_ = w.sm.ProcessEvent(ctx, taskID, event)
}
