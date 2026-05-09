package repository

import (
	"context"
	"database/sql"
	"errors"
	"time"
)

var (
	ErrTaskNotFound = errors.New("task not found")
)

type contextKey string

const (
	OrgIDKey contextKey = "organization_id"
)

func OrganizationIDFromContext(ctx context.Context) string {
	if val, ok := ctx.Value(OrgIDKey).(string); ok {
		return val
	}
	// Fallback to plain string for compatibility with existing tests
	if val, ok := ctx.Value("organization_id").(string); ok {
		return val
	}
	return ""
}

type TaskRepository interface {
	CreateTask(ctx context.Context, task *Task) error
	GetTasksByOrg(ctx context.Context, orgID string) ([]*Task, error)
	UpdateTaskStatus(ctx context.Context, taskID string, status string) error
}

type sqlTaskRepository struct {
	db *sql.DB
}

func NewSQLTaskRepository(db *sql.DB) TaskRepository {
	return &sqlTaskRepository{db: db}
}

func (r *sqlTaskRepository) CreateTask(ctx context.Context, task *Task) error {
	orgID := OrganizationIDFromContext(ctx)
	if orgID != "" {
		task.OrganizationID = orgID
	}

	if task.CreatedAt.IsZero() {
		task.CreatedAt = time.Now()
	}
	if task.UpdatedAt.IsZero() {
		task.UpdatedAt = time.Now()
	}

	query := `
		INSERT INTO tasks (id, organization_id, parent_task_id, title, description, status, assigned_agent_role, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
	`
	_, err := r.db.ExecContext(ctx, query,
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

func (r *sqlTaskRepository) GetTasksByOrg(ctx context.Context, orgID string) ([]*Task, error) {
	ctxOrgID := OrganizationIDFromContext(ctx)
	if ctxOrgID != "" && ctxOrgID != orgID {
		return nil, errors.New("unauthorized organization access")
	}

	query := `
		SELECT id, organization_id, parent_task_id, title, description, status, assigned_agent_role, created_at, updated_at
		FROM tasks
		WHERE organization_id = $1
	`
	rows, err := r.db.QueryContext(ctx, query, orgID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*Task
	for rows.Next() {
		t := &Task{}
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
	return tasks, nil
}

func (r *sqlTaskRepository) UpdateTaskStatus(ctx context.Context, taskID string, status string) error {
	orgID := OrganizationIDFromContext(ctx)
	query := `
		UPDATE tasks
		SET status = $1, updated_at = $2
		WHERE id = $3 AND organization_id = $4
	`
	res, err := r.db.ExecContext(ctx, query, status, time.Now(), taskID, orgID)
	if err != nil {
		return err
	}
	rowsAffected, err := res.RowsAffected()
	if err != nil {
		return err
	}
	if rowsAffected == 0 {
		return ErrTaskNotFound
	}
	return nil
}
