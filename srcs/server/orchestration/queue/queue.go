package queue

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

type Job struct {
	ID        string          `json:"id"`
	Type      string          `json:"type"`
	Payload   json.RawMessage `json:"payload"`
	Status    string          `json:"status"`
	CreatedAt time.Time       `json:"created_at"`
	UpdatedAt time.Time       `json:"updated_at"`
}

type TaskQueue interface {
	Enqueue(ctx context.Context, job *Job) error
	Dequeue(ctx context.Context, jobTypes []string) (*Job, error)
	Complete(ctx context.Context, jobID string) error
	Fail(ctx context.Context, jobID string, reason string) error
}

type SQLiteTaskQueue struct {
	db *sql.DB
}

func NewSQLiteTaskQueue(db *sql.DB) (*SQLiteTaskQueue, error) {
	_, err := db.Exec(`
		CREATE TABLE IF NOT EXISTS sub_agent_jobs (
			id TEXT PRIMARY KEY,
			type TEXT NOT NULL,
			payload JSON,
			status TEXT NOT NULL,
			created_at DATETIME,
			updated_at DATETIME
		)
	`)
	if err != nil {
		return nil, fmt.Errorf("failed to create jobs table: %w", err)
	}
	return &SQLiteTaskQueue{db: db}, nil
}

func (q *SQLiteTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	now := time.Now()
	_, err := q.db.ExecContext(ctx, `
		INSERT INTO sub_agent_jobs (id, type, payload, status, created_at, updated_at)
		VALUES (?, ?, ?, ?, ?, ?)
	`, job.ID, job.Type, job.Payload, "PENDING", now, now)
	return err
}

func (q *SQLiteTaskQueue) Dequeue(ctx context.Context, jobTypes []string) (*Job, error) {
	tx, err := q.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var job Job
	var payloadStr string
	var query string
	var args []interface{}

	if len(jobTypes) > 0 {
		query = "SELECT id, type, payload, status, created_at, updated_at FROM sub_agent_jobs WHERE status = 'PENDING' AND type = ? ORDER BY created_at ASC LIMIT 1"
		args = []interface{}{jobTypes[0]}
	} else {
		query = "SELECT id, type, payload, status, created_at, updated_at FROM sub_agent_jobs WHERE status = 'PENDING' ORDER BY created_at ASC LIMIT 1"
	}

	err = tx.QueryRowContext(ctx, query, args...).Scan(&job.ID, &job.Type, &payloadStr, &job.Status, &job.CreatedAt, &job.UpdatedAt)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}

	job.Payload = json.RawMessage(payloadStr)

	_, err = tx.ExecContext(ctx, "UPDATE sub_agent_jobs SET status = 'PROCESSING', updated_at = ? WHERE id = ?", time.Now(), job.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	job.Status = "PROCESSING"
	return &job, nil
}

func (q *SQLiteTaskQueue) Complete(ctx context.Context, jobID string) error {
	_, err := q.db.ExecContext(ctx, "UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = ? WHERE id = ?", time.Now(), jobID)
	return err
}

func (q *SQLiteTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	_, err := q.db.ExecContext(ctx, "UPDATE sub_agent_jobs SET status = 'FAILED', updated_at = ? WHERE id = ?", time.Now(), jobID)
	return err
}

type RedisTaskQueue struct {
}

func NewRedisTaskQueue() *RedisTaskQueue {
	return &RedisTaskQueue{}
}

func (q *RedisTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	return nil
}

func (q *RedisTaskQueue) Dequeue(ctx context.Context, jobTypes []string) (*Job, error) {
	return nil, nil
}

func (q *RedisTaskQueue) Complete(ctx context.Context, jobID string) error {
	return nil
}

func (q *RedisTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	return nil
}
