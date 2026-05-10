package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"time"

	"github.com/google/uuid"
)

// Task represents a task in the sub-agent queue
type Task struct {
	ID             string          `json:"id"`
	OrganizationID string          `json:"organization_id"`
	ParentTaskID   string          `json:"parent_task_id"`
	Payload        json.RawMessage `json:"payload"`
	Status         string          `json:"status"`
	WorkerID       string          `json:"worker_id"`
	CreatedAt      time.Time       `json:"created_at"`
	UpdatedAt      time.Time       `json:"updated_at"`
}

// TaskQueue is the interface for interacting with the sub-agent queue
type TaskQueue interface {
	Enqueue(ctx context.Context, task *Task) error
	Dequeue(ctx context.Context) (*Task, error)
	Acknowledge(ctx context.Context, taskID string) error
	FailTask(ctx context.Context, taskID string) error
}

// DBTaskQueue is a database-backed implementation of TaskQueue
type DBTaskQueue struct {
	db       *sql.DB
	isPgSQL  bool
	workerID string
}

// NewDBTaskQueue creates a new DBTaskQueue
func NewDBTaskQueue(db *sql.DB, isPgSQL bool, workerID string) *DBTaskQueue {
	if workerID == "" {
		workerID = uuid.New().String()
	}
	return &DBTaskQueue{
		db:       db,
		isPgSQL:  isPgSQL,
		workerID: workerID,
	}
}

// Enqueue adds a new task to the queue
func (q *DBTaskQueue) Enqueue(ctx context.Context, task *Task) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	task.Status = "QUEUED"
	now := time.Now()
	task.CreatedAt = now
	task.UpdatedAt = now

	var payloadStr interface{}
	if len(task.Payload) > 0 {
		payloadStr = string(task.Payload)
	} else {
		payloadStr = nil
	}

	query := `
		INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)`

	if !q.isPgSQL {
		query = `
			INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, created_at, updated_at)
			VALUES (?, ?, ?, ?, ?, ?, ?)`
	}

	_, err := q.db.ExecContext(ctx, query,
		task.ID,
		task.OrganizationID,
		task.ParentTaskID,
		payloadStr,
		task.Status,
		task.CreatedAt,
		task.UpdatedAt,
	)

	return err
}

// Dequeue atomically retrieves and locks a queued task
func (q *DBTaskQueue) Dequeue(ctx context.Context) (*Task, error) {
	var task Task
	var payloadStr sql.NullString
	var workerIDStr sql.NullString

	tx, err := q.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	if q.isPgSQL {
		query := `
			UPDATE sub_agent_queue
			SET status = 'RUNNING', worker_id = $1, updated_at = NOW()
			WHERE id = (
				SELECT id FROM sub_agent_queue
				WHERE status = 'QUEUED'
				ORDER BY created_at ASC
				FOR UPDATE SKIP LOCKED
				LIMIT 1
			)
			RETURNING id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at`

		err = tx.QueryRowContext(ctx, query, q.workerID).Scan(
			&task.ID,
			&task.OrganizationID,
			&task.ParentTaskID,
			&payloadStr,
			&task.Status,
			&workerIDStr,
			&task.CreatedAt,
			&task.UpdatedAt,
		)
	} else {
		// SQLite doesn't support SKIP LOCKED, so we do it in two steps within a transaction
		selectQuery := `
			SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
			FROM sub_agent_queue
			WHERE status = 'QUEUED'
			ORDER BY created_at ASC
			LIMIT 1`

		err = tx.QueryRowContext(ctx, selectQuery).Scan(
			&task.ID,
			&task.OrganizationID,
			&task.ParentTaskID,
			&payloadStr,
			&task.Status,
			&workerIDStr,
			&task.CreatedAt,
			&task.UpdatedAt,
		)

		if err == nil {
			updateQuery := `
				UPDATE sub_agent_queue
				SET status = 'RUNNING', worker_id = ?, updated_at = CURRENT_TIMESTAMP
				WHERE id = ? AND status = 'QUEUED'`
			res, execErr := tx.ExecContext(ctx, updateQuery, q.workerID, task.ID)
			err = execErr

			if err == nil {
				rowsAffected, _ := res.RowsAffected()
				if rowsAffected == 1 {
					task.Status = "RUNNING"
					task.WorkerID = q.workerID
					task.UpdatedAt = time.Now()
				} else {
					// Someone else grabbed it before us, rollback and return nothing
					tx.Rollback()
					return nil, nil
				}
			}
		}
	}

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No tasks available
		}
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	if payloadStr.Valid {
		task.Payload = json.RawMessage(payloadStr.String)
	}
	if workerIDStr.Valid {
		task.WorkerID = workerIDStr.String
	}

	return &task, nil
}

// Acknowledge marks a task as COMPLETED
func (q *DBTaskQueue) Acknowledge(ctx context.Context, taskID string) error {
	query := `
		UPDATE sub_agent_queue
		SET status = 'COMPLETED', updated_at = NOW()
		WHERE id = $1`

	if !q.isPgSQL {
		query = `
			UPDATE sub_agent_queue
			SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
			WHERE id = ?`
	}

	res, err := q.db.ExecContext(ctx, query, taskID)
	if err != nil {
		return err
	}

	rows, err := res.RowsAffected()
	if err != nil {
		return err
	}

	if rows == 0 {
		return errors.New("task not found or not updated")
	}

	return nil
}

// FailTask marks a task as FAILED so it doesn't get stuck in RUNNING
func (q *DBTaskQueue) FailTask(ctx context.Context, taskID string) error {
	query := `
		UPDATE sub_agent_queue
		SET status = 'FAILED', updated_at = NOW()
		WHERE id = $1`

	if !q.isPgSQL {
		query = `
			UPDATE sub_agent_queue
			SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP
			WHERE id = ?`
	}

	res, err := q.db.ExecContext(ctx, query, taskID)
	if err != nil {
		return err
	}

	rows, err := res.RowsAffected()
	if err != nil {
		return err
	}

	if rows == 0 {
		return errors.New("task not found or not updated")
	}

	return nil
}
