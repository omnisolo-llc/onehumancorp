package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"
)

type SubAgentTask struct {
	ID             string
	OrganizationID string
	ParentTaskID   string
	Payload        json.RawMessage
	Status         string
	WorkerID       *string
	CreatedAt      time.Time
	UpdatedAt      time.Time
}

type TaskQueue interface {
	Enqueue(ctx context.Context, task *SubAgentTask) error
	Dequeue(ctx context.Context, workerID string) (*SubAgentTask, error)
	Acknowledge(ctx context.Context, taskID string, status string) error
}

type PostgresTaskQueue struct {
	db *sql.DB
}

func NewPostgresTaskQueue(db *sql.DB) *PostgresTaskQueue {
	return &PostgresTaskQueue{db: db}
}

func (q *PostgresTaskQueue) Enqueue(ctx context.Context, task *SubAgentTask) error {
	query := `
		INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`
	_, err := q.db.ExecContext(ctx, query,
		task.ID, task.OrganizationID, task.ParentTaskID, task.Payload,
		task.Status, task.WorkerID, task.CreatedAt, task.UpdatedAt)
	return err
}

func (q *PostgresTaskQueue) Dequeue(ctx context.Context, workerID string) (*SubAgentTask, error) {
	tx, err := q.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	query := `
		SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
		FROM sub_agent_queue
		WHERE status = 'PENDING'
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`
	row := tx.QueryRowContext(ctx, query)

	var task SubAgentTask
	err = row.Scan(&task.ID, &task.OrganizationID, &task.ParentTaskID, &task.Payload,
		&task.Status, &task.WorkerID, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No task available
		}
		return nil, err
	}

	updateQuery := `
		UPDATE sub_agent_queue
		SET status = 'IN_PROGRESS', worker_id = $1, updated_at = NOW()
		WHERE id = $2
	`
	_, err = tx.ExecContext(ctx, updateQuery, workerID, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	task.WorkerID = &workerID
	return &task, nil
}

func (q *PostgresTaskQueue) Acknowledge(ctx context.Context, taskID string, status string) error {
	query := `
		UPDATE sub_agent_queue
		SET status = $1, updated_at = NOW()
		WHERE id = $2
	`
	_, err := q.db.ExecContext(ctx, query, status, taskID)
	return err
}

type SqliteTaskQueue struct {
	db *sql.DB
}

func NewSqliteTaskQueue(db *sql.DB) *SqliteTaskQueue {
	return &SqliteTaskQueue{db: db}
}

func (q *SqliteTaskQueue) Enqueue(ctx context.Context, task *SubAgentTask) error {
	query := `
		INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)
	`
	_, err := q.db.ExecContext(ctx, query,
		task.ID, task.OrganizationID, task.ParentTaskID, string(task.Payload),
		task.Status, task.WorkerID, task.CreatedAt, task.UpdatedAt)
	return err
}

func (q *SqliteTaskQueue) Dequeue(ctx context.Context, workerID string) (*SubAgentTask, error) {
	// SQLite lacks FOR UPDATE SKIP LOCKED. For simplicity, we use an immediate update with a subquery
	// Note: In real SQLite production we often use a single writer or a simpler lock.
	tx, err := q.db.BeginTx(ctx, &sql.TxOptions{Isolation: sql.LevelSerializable})
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	query := `
		SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
		FROM sub_agent_queue
		WHERE status = 'PENDING'
		LIMIT 1
	`
	row := tx.QueryRowContext(ctx, query)

	var task SubAgentTask
	var payloadStr string
	err = row.Scan(&task.ID, &task.OrganizationID, &task.ParentTaskID, &payloadStr,
		&task.Status, &task.WorkerID, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No task available
		}
		return nil, err
	}
	task.Payload = json.RawMessage(payloadStr)

	updateQuery := `
		UPDATE sub_agent_queue
		SET status = 'IN_PROGRESS', worker_id = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ? AND status = 'PENDING'
	`
	res, err := tx.ExecContext(ctx, updateQuery, workerID, task.ID)
	if err != nil {
		return nil, err
	}
	rowsAffected, _ := res.RowsAffected()
	if rowsAffected == 0 {
		return nil, nil // Another worker grabbed it
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	task.WorkerID = &workerID
	return &task, nil
}

func (q *SqliteTaskQueue) Acknowledge(ctx context.Context, taskID string, status string) error {
	query := `
		UPDATE sub_agent_queue
		SET status = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ?
	`
	_, err := q.db.ExecContext(ctx, query, status, taskID)
	return err
}
