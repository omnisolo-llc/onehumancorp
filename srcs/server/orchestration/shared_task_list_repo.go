package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SharedTaskListRepo struct {
	dbProvider db.Provider
}

func NewSharedTaskListRepo(dbProvider db.Provider) *SharedTaskListRepo {
	return &SharedTaskListRepo{
		dbProvider: dbProvider,
	}
}

func (r *SharedTaskListRepo) CreateTask(ctx context.Context, organizationID, epicID, title string, payload json.RawMessage, deps []string) (*SharedTaskListTask, error) {
	if organizationID == "" {
		return nil, fmt.Errorf("organization_id is required")
	}
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	task := &SharedTaskListTask{
		ID:        uuid.New().String(),
		OrganizationID: organizationID,
		EpicID:    epicID,
		Title:     title,
		Status:    "PENDING",
		Payload:   payload,
		CreatedAt: time.Now().UTC(),
		UpdatedAt: time.Now().UTC(),
	}

	var payloadVal interface{}
	if task.Payload != nil {
		payloadVal = string(task.Payload)
	}

	_, err = tx.Exec(ctx, `
		INSERT INTO shared_task_list_tasks (id, organization_id, epic_id, title, status, payload, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`, task.ID, task.OrganizationID, task.EpicID, task.Title, task.Status, payloadVal, task.CreatedAt, task.UpdatedAt)

	if err != nil {
		return nil, fmt.Errorf("failed to insert task: %w", err)
	}

	for _, depID := range deps {
		_, err = tx.Exec(ctx, `
			INSERT INTO shared_task_list_dependencies (task_id, depends_on_task_id)
			VALUES ($1, $2)
		`, task.ID, depID)
		if err != nil {
			return nil, fmt.Errorf("failed to insert task dependency: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return task, nil
}

func (r *SharedTaskListRepo) UpdateTaskStatus(ctx context.Context, organizationID, taskID, status string) error {
	if organizationID == "" {
		return fmt.Errorf("organization_id is required")
	}
	_, err := r.dbProvider.Exec(ctx, `
		UPDATE shared_task_list_tasks
		SET status = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND organization_id = $3
	`, status, taskID, organizationID)
	return err
}

func (r *SharedTaskListRepo) GetNextAvailableTask(ctx context.Context, organizationID, agentID string) (*SharedTaskListTask, error) {
	if organizationID == "" {
		return nil, fmt.Errorf("organization_id is required")
	}
	if r.dbProvider.IsSQLite() {
		return r.getNextAvailableTaskSQLite(ctx, organizationID, agentID)
	}
	return r.getNextAvailableTaskPostgres(ctx, organizationID, agentID)
}

func (r *SharedTaskListRepo) getNextAvailableTaskSQLite(ctx context.Context, organizationID, agentID string) (*SharedTaskListTask, error) {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		SELECT t.id, t.organization_id, t.epic_id, t.title, t.status, t.payload, t.created_at, t.updated_at
		FROM shared_task_list_tasks t
		WHERE t.organization_id = $1 AND t.status = 'PENDING' AND (t.locked_by IS NULL OR t.locked_at < datetime('now', '-5 minutes'))
		AND NOT EXISTS (
			SELECT 1 FROM shared_task_list_dependencies d
			JOIN shared_task_list_tasks dep_t ON d.depends_on_task_id = dep_t.id
			WHERE d.task_id = t.id AND dep_t.status != 'COMPLETED'
		)
		LIMIT 1
	`
	row := tx.QueryRow(ctx, query, organizationID)

	var task SharedTaskListTask
	var payloadStr *string
	if err := row.Scan(&task.ID, &task.OrganizationID, &task.EpicID, &task.Title, &task.Status, &payloadStr, &task.CreatedAt, &task.UpdatedAt); err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	if payloadStr != nil && *payloadStr != "" {
		task.Payload = json.RawMessage(*payloadStr)
	}

	now := time.Now().UTC()
	task.LockedBy = &agentID
	task.LockedAt = &now
	task.Status = "IN_PROGRESS"

	_, err = tx.Exec(ctx, `
		UPDATE shared_task_list_tasks
		SET status = 'IN_PROGRESS', locked_by = $1, locked_at = $2, updated_at = $2
		WHERE id = $3 AND organization_id = $4
	`, agentID, now, task.ID, task.OrganizationID)

	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return &task, nil
}

func (r *SharedTaskListRepo) getNextAvailableTaskPostgres(ctx context.Context, organizationID, agentID string) (*SharedTaskListTask, error) {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		SELECT t.id, t.organization_id, t.epic_id, t.title, t.status, t.payload, t.created_at, t.updated_at
		FROM shared_task_list_tasks t
		WHERE t.organization_id = $1 AND t.status = 'PENDING' AND (t.locked_by IS NULL OR t.locked_at < NOW() - INTERVAL '5 minutes')
		AND NOT EXISTS (
			SELECT 1 FROM shared_task_list_dependencies d
			JOIN shared_task_list_tasks dep_t ON d.depends_on_task_id = dep_t.id
			WHERE d.task_id = t.id AND dep_t.status != 'COMPLETED'
		)
		LIMIT 1
		FOR UPDATE SKIP LOCKED
	`
	row := tx.QueryRow(ctx, query, organizationID)

	var task SharedTaskListTask
	var payloadStr *string
	if err := row.Scan(&task.ID, &task.OrganizationID, &task.EpicID, &task.Title, &task.Status, &payloadStr, &task.CreatedAt, &task.UpdatedAt); err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	if payloadStr != nil && *payloadStr != "" {
		task.Payload = json.RawMessage(*payloadStr)
	}

	now := time.Now().UTC()
	task.LockedBy = &agentID
	task.LockedAt = &now
	task.Status = "IN_PROGRESS"

	_, err = tx.Exec(ctx, `
		UPDATE shared_task_list_tasks
		SET status = 'IN_PROGRESS', locked_by = $1, locked_at = $2, updated_at = $2
		WHERE id = $3 AND organization_id = $4
	`, agentID, now, task.ID, task.OrganizationID)

	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return &task, nil
}
