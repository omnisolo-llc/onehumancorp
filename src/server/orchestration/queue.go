package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"
)

// Task represents a job in the sub-agent queue.
type Task struct {
	ID             string          `json:"id"`
	OrganizationID string          `json:"organization_id"`
	ParentTaskID   string          `json:"parent_task_id"`
	Payload        json.RawMessage `json:"payload"`
	Status         string          `json:"status"`
	WorkerID       *string         `json:"worker_id"`
	CreatedAt      time.Time       `json:"created_at"`
	UpdatedAt      time.Time       `json:"updated_at"`
}

// TaskQueue represents the interface for interacting with the sub-agent queue.
type TaskQueue interface {
	Enqueue(ctx context.Context, task *Task) error
	Dequeue(ctx context.Context) (*Task, error)
	Acknowledge(ctx context.Context, taskID string) error
}

// AgentHarness is the interface for executing a dequeued task.
type AgentHarness interface {
	Execute(ctx context.Context, task *Task) error
}

// DBTaskQueue implements TaskQueue using a generic database backend (SQLite or Postgres).
type DBTaskQueue struct {
	db       *sql.DB
	workerID string
	isSQLite bool
}

// NewDBTaskQueue creates a new DBTaskQueue instance.
func NewDBTaskQueue(db *sql.DB, workerID string, isSQLite bool) *DBTaskQueue {
	return &DBTaskQueue{
		db:       db,
		workerID: workerID,
		isSQLite: isSQLite,
	}
}

// Enqueue adds a new task to the queue.
func (q *DBTaskQueue) Enqueue(ctx context.Context, task *Task) error {
	query := `
		INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, created_at, updated_at)
		VALUES ($1, $2, $3, $4, 'QUEUED', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`
	_, err := q.db.ExecContext(ctx, query, task.ID, task.OrganizationID, task.ParentTaskID, task.Payload)
	return err
}

// Dequeue atomically retrieves and locks a pending task.
func (q *DBTaskQueue) Dequeue(ctx context.Context) (*Task, error) {
	task := &Task{}
	var payload []byte

	tx, err := q.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	if !q.isSQLite {
		// Postgres approach
		pgQuery := `
			UPDATE sub_agent_queue
			SET status = 'PROCESSING', worker_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id
				FROM sub_agent_queue
				WHERE status = 'QUEUED'
				ORDER BY created_at ASC
				FOR UPDATE SKIP LOCKED
				LIMIT 1
			)
			RETURNING id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
		`

		err = tx.QueryRowContext(ctx, pgQuery, q.workerID).Scan(
			&task.ID,
			&task.OrganizationID,
			&task.ParentTaskID,
			&payload,
			&task.Status,
			&task.WorkerID,
			&task.CreatedAt,
			&task.UpdatedAt,
		)

		if err == sql.ErrNoRows {
			return nil, nil // No tasks in queue
		} else if err != nil {
			return nil, err
		}

		task.Payload = json.RawMessage(payload)
		return task, tx.Commit()
	}

	// SQLite approach (two-step optimistic locking)
	// Find a queued item first
	findQuery := `SELECT id FROM sub_agent_queue WHERE status = 'QUEUED' ORDER BY created_at ASC LIMIT 1`
	var id string
	err = tx.QueryRowContext(ctx, findQuery).Scan(&id)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No tasks in queue
		}
		return nil, err
	}

	// Attempt to claim it
	updateQuery := `
		UPDATE sub_agent_queue
		SET status = 'PROCESSING', worker_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'QUEUED'
	`
	res, err := tx.ExecContext(ctx, updateQuery, q.workerID, id)
	if err != nil {
		return nil, err
	}

	rowsAffected, err := res.RowsAffected()
	if err != nil {
		return nil, err
	}

	if rowsAffected == 0 {
		// Someone else claimed it, that's fine, we return nil so caller tries again later
		return nil, nil
	}

	// Fetch full details of claimed task
	fetchQuery := `
		SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
		FROM sub_agent_queue
		WHERE id = $1
	`
	err = tx.QueryRowContext(ctx, fetchQuery, id).Scan(
		&task.ID,
		&task.OrganizationID,
		&task.ParentTaskID,
		&payload,
		&task.Status,
		&task.WorkerID,
		&task.CreatedAt,
		&task.UpdatedAt,
	)
	if err != nil {
		return nil, err
	}

	task.Payload = json.RawMessage(payload)
	return task, tx.Commit()
}

// Acknowledge marks a task as COMPLETED.
func (q *DBTaskQueue) Acknowledge(ctx context.Context, taskID string) error {
	query := `
		UPDATE sub_agent_queue
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND worker_id = $2
	`
	_, err := q.db.ExecContext(ctx, query, taskID, q.workerID)
	return err
}

// WorkerPool manages polling tasks and distributing them to agents.
type WorkerPool struct {
	queue       TaskQueue
	harness     AgentHarness
	concurrency int
	cancel      context.CancelFunc
}

// NewWorkerPool creates a new WorkerPool.
func NewWorkerPool(queue TaskQueue, harness AgentHarness, concurrency int) *WorkerPool {
	return &WorkerPool{
		queue:       queue,
		harness:     harness,
		concurrency: concurrency,
	}
}

// Start begins polling for tasks with the specified concurrency limit.
func (wp *WorkerPool) Start(ctx context.Context) {
	ctx, wp.cancel = context.WithCancel(ctx)
	for i := 0; i < wp.concurrency; i++ {
		go wp.pollLoop(ctx)
	}
}

// Stop gracefully shuts down the worker pool.
func (wp *WorkerPool) Stop() {
	if wp.cancel != nil {
		wp.cancel()
	}
}

func (wp *WorkerPool) pollLoop(ctx context.Context) {
	ticker := time.NewTicker(10 * time.Millisecond)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		default:
			// Non-blocking select to allow continuous polling without waiting for ticker
		}

		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			task, err := wp.queue.Dequeue(ctx)
			if err != nil {
				// Wait and retry on error
				time.Sleep(10 * time.Millisecond)
				continue
			}

			if task == nil {
				continue // No task available
			}

			// Execute task
			err = wp.harness.Execute(ctx, task)
			if err == nil {
				// Acknowledge upon success
				_ = wp.queue.Acknowledge(ctx, task.ID)
			}
			// In a more complex scenario, we would handle retries / failures here.
		}
	}
}
