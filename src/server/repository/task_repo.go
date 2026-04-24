package repository

import (
	"context"
	"database/sql"
	"errors"
	"time"

	"github.com/google/uuid"
)

type TaskRepository struct {
	DB *sql.DB
}

func NewTaskRepository(db *sql.DB) *TaskRepository {
	return &TaskRepository{DB: db}
}

// OrganizationIDFromContext extracts organization ID from context
// For the sake of this mock we will define it here, but in a real app this is usually in an auth package
type contextKey string
const orgIDKey contextKey = "organization_id"

func OrganizationIDFromContext(ctx context.Context) (string, error) {
	orgID, ok := ctx.Value(orgIDKey).(string)
	if !ok || orgID == "" {
		return "", errors.New("organization ID not found in context")
	}
	return orgID, nil
}

func (r *TaskRepository) CreateTask(ctx context.Context, task *Task) error {
	orgID, err := OrganizationIDFromContext(ctx)
	if err != nil {
		return err
	}
	task.OrganizationID = orgID

	if task.ID == "" {
		task.ID = uuid.New().String()
	}

	now := time.Now()
	if task.CreatedAt.IsZero() {
		task.CreatedAt = now
	}
	task.UpdatedAt = now

	query := `
		INSERT INTO tasks (id, organization_id, parent_task_id, title, description, status, assigned_agent_role, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
	`
	_, err = r.DB.ExecContext(ctx, query,
		task.ID,
		task.OrganizationID,
		task.ParentTaskID,
		task.Title,
		task.Description,
		task.Status,
		task.AssignedAgentRole,
		task.CreatedAt,
		task.UpdatedAt,
	)
	return err
}

func (r *TaskRepository) GetTasksByOrg(ctx context.Context) ([]Task, error) {
	orgID, err := OrganizationIDFromContext(ctx)
	if err != nil {
		return nil, err
	}

	query := `
		SELECT id, organization_id, parent_task_id, title, description, status, assigned_agent_role, created_at, updated_at
		FROM tasks
		WHERE organization_id = $1
	`
	rows, err := r.DB.QueryContext(ctx, query, orgID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []Task
	for rows.Next() {
		var t Task
		err := rows.Scan(
			&t.ID,
			&t.OrganizationID,
			&t.ParentTaskID,
			&t.Title,
			&t.Description,
			&t.Status,
			&t.AssignedAgentRole,
			&t.CreatedAt,
			&t.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
		tasks = append(tasks, t)
	}
	if err = rows.Err(); err != nil {
		return nil, err
	}
	return tasks, nil
}

func (r *TaskRepository) UpdateTaskStatus(ctx context.Context, taskID, status string) error {
	orgID, err := OrganizationIDFromContext(ctx)
	if err != nil {
		return err
	}

	query := `
		UPDATE tasks
		SET status = $1, updated_at = $2
		WHERE id = $3 AND organization_id = $4
	`
	// Verify current state before update to avoid relying on RowsAffected
	var count int
	checkQuery := "SELECT COUNT(*) FROM tasks WHERE id = $1 AND organization_id = $2"
	err = r.DB.QueryRowContext(ctx, checkQuery, taskID, orgID).Scan(&count)
	if err != nil {
		return errors.New("task not found or not owned by organization")
	}
	if count == 0 {
		return errors.New("task not found or not owned by organization")
	}

	_, err = r.DB.ExecContext(ctx, query, status, time.Now(), taskID, orgID)
	if err != nil {
		return err
	}

	return nil
}
