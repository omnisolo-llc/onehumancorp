package mcp

import (
	"context"
	"database/sql"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type AsyncTask struct {
	ID        string
	TenantID  string
	AgentID   string
	Status    string
	Payload   string
	CreatedAt string
}

type AsyncTaskTracker struct {
	pool db.Provider
}

func NewAsyncTaskTracker(pool db.Provider) *AsyncTaskTracker {
	return &AsyncTaskTracker{pool: pool}
}

func (t *AsyncTaskTracker) CreateTask(ctx context.Context, task AsyncTask) error {
	query := `
		INSERT INTO mcp_async_tasks (id, tenant_id, agent_id, status, payload)
		VALUES ($1, $2, $3, $4, $5)
	`
	_, err := t.pool.Exec(ctx, query, task.ID, task.TenantID, task.AgentID, task.Status, task.Payload)
	return err
}

func (t *AsyncTaskTracker) GetTask(ctx context.Context, id string) (*AsyncTask, error) {
	query := `
		SELECT id, tenant_id, agent_id, status, payload, created_at
		FROM mcp_async_tasks
		WHERE id = $1
	`
	var task AsyncTask
	var payload sql.NullString
	err := t.pool.QueryRow(ctx, query, id).Scan(
		&task.ID,
		&task.TenantID,
		&task.AgentID,
		&task.Status,
		&payload,
		&task.CreatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil
		}
		return nil, err
	}
	if payload.Valid {
		task.Payload = payload.String
	}
	return &task, nil
}

func (t *AsyncTaskTracker) UpdateTaskStatus(ctx context.Context, id, status, payload string) error {
	query := `
		UPDATE mcp_async_tasks
		SET status = $1, payload = $2
		WHERE id = $3
	`
	rowsAffected, err := t.pool.Exec(ctx, query, status, payload, id)
	if err != nil {
		return err
	}
	if rowsAffected == 0 {
		return fmt.Errorf("task not found: %s", id)
	}
	return nil
}