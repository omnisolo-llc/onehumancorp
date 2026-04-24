package db

import (
	"context"
)

// SharedTaskRepository defines the interface for managing shared tasks.
type SharedTaskRepository interface {
	AcquireTask(ctx context.Context, organizationID, agentID string) (*TaskRecord, error)
	CreateTask(ctx context.Context, organizationID string, task *TaskRecord, title string) error
	GetTask(ctx context.Context, organizationID, taskID string) (*TaskRecord, error)
	UpdateTask(ctx context.Context, organizationID string, task *TaskRecord) error
	DeleteTask(ctx context.Context, organizationID, taskID string) error
}

// sharedTaskRepositoryImpl implements SharedTaskRepository using Provider.
type sharedTaskRepositoryImpl struct {
	provider Provider
}

// NewSharedTaskRepository creates a new SharedTaskRepository.
func NewSharedTaskRepository(provider Provider) SharedTaskRepository {
	return &sharedTaskRepositoryImpl{
		provider: provider,
	}
}

// AcquireTask delegates to the underlying db Provider.
func (r *sharedTaskRepositoryImpl) AcquireTask(ctx context.Context, organizationID, agentID string) (*TaskRecord, error) {
	return r.provider.AcquireTask(ctx, organizationID, agentID)
}


// CreateTask creates a new task in the database.
func (r *sharedTaskRepositoryImpl) CreateTask(ctx context.Context, organizationID string, task *TaskRecord, title string) error {
	query := `
		INSERT INTO shared_tasks (id, organization_id, title, status, payload, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
	_, err := r.provider.Exec(ctx, query, task.ID, organizationID, title, task.Status, task.Payload, task.CreatedAt, task.UpdatedAt)
	return err
}

// GetTask retrieves a task by ID.
func (r *sharedTaskRepositoryImpl) GetTask(ctx context.Context, organizationID, taskID string) (*TaskRecord, error) {
	query := `
		SELECT id, parent_plan_id, assigned_agent_id, status, payload, created_at, updated_at
		FROM shared_tasks
		WHERE id = $1 AND organization_id = $2
	`
	var t TaskRecord
	err := r.provider.QueryRow(ctx, query, taskID, organizationID).Scan(
		&t.ID, &t.ParentTaskID, &t.AgentID, &t.Status, &t.Payload, &t.CreatedAt, &t.UpdatedAt,
	)
	if err != nil {
		return nil, err
	}
	return &t, nil
}

// UpdateTask updates an existing task in the database.
func (r *sharedTaskRepositoryImpl) UpdateTask(ctx context.Context, organizationID string, task *TaskRecord) error {
	query := `
		UPDATE shared_tasks
		SET status = $1, assigned_agent_id = $2, payload = $3, updated_at = $4
		WHERE id = $5 AND organization_id = $6
	`
	_, err := r.provider.Exec(ctx, query, task.Status, task.AgentID, task.Payload, task.UpdatedAt, task.ID, organizationID)
	return err
}

// DeleteTask deletes a task from the database.
func (r *sharedTaskRepositoryImpl) DeleteTask(ctx context.Context, organizationID, taskID string) error {
	query := `
		DELETE FROM shared_tasks
		WHERE id = $1 AND organization_id = $2
	`
	_, err := r.provider.Exec(ctx, query, taskID, organizationID)
	return err
}
