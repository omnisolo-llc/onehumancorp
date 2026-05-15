package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/google/uuid"
)

type SharedTaskV4 struct {
	ID              string
	OrganizationID  string
	Title           string
	Description     string
	Status          string
	AgentID         string
	Priority        string
	Payload         string
	ParentPlanID    string
	Dependencies    string
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

type SharedTaskOrchestrator struct {
	db *sql.DB
}

func NewSharedTaskOrchestrator(db *sql.DB) *SharedTaskOrchestrator {
	return &SharedTaskOrchestrator{db: db}
}

func (o *SharedTaskOrchestrator) CreateTask(ctx context.Context, task *SharedTaskV4) (*SharedTaskV4, error) {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	if task.CreatedAt.IsZero() {
		task.CreatedAt = time.Now()
	}
	if task.UpdatedAt.IsZero() {
		task.UpdatedAt = time.Now()
	}

	query := `
		INSERT INTO shared_tasks_v4 (
			id, organization_id, title, description, status, agent_id,
			priority, payload, parent_plan_id, dependencies, created_at, updated_at
		) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
	`
	_, err := o.db.ExecContext(ctx, query,
		task.ID, task.OrganizationID, task.Title, task.Description, task.Status, task.AgentID,
		task.Priority, task.Payload, task.ParentPlanID, task.Dependencies, task.CreatedAt, task.UpdatedAt,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create task: %w", err)
	}

	return task, nil
}

func (o *SharedTaskOrchestrator) GetTask(ctx context.Context, id string) (*SharedTaskV4, error) {
	query := `SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at FROM shared_tasks_v4 WHERE id = $1`
	row := o.db.QueryRowContext(ctx, query, id)

	var task SharedTaskV4
	var createdAt, updatedAt string
	err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AgentID,
		&task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies, &createdAt, &updatedAt,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to get task: %w", err)
	}

	t1, _ := time.Parse(time.RFC3339Nano, createdAt)
	if t1.IsZero() {
		t1, _ = time.Parse("2006-01-02 15:04:05-07:00", createdAt)
	}
	if t1.IsZero() {
		t1, _ = time.Parse("2006-01-02 15:04:05", createdAt)
	}
	task.CreatedAt = t1

	t2, _ := time.Parse(time.RFC3339Nano, updatedAt)
	if t2.IsZero() {
		t2, _ = time.Parse("2006-01-02 15:04:05-07:00", updatedAt)
	}
	if t2.IsZero() {
		t2, _ = time.Parse("2006-01-02 15:04:05", updatedAt)
	}
	task.UpdatedAt = t2

	return &task, nil
}

// DeleteTask removes a task from the orchestrator.
func (o *SharedTaskOrchestrator) DeleteTask(ctx context.Context, id string) error {
	query := `DELETE FROM shared_tasks_v4 WHERE id = $1`
	_, err := o.db.ExecContext(ctx, query, id)
	return err
}

// UpdateTask updates a task in the orchestrator.
func (o *SharedTaskOrchestrator) UpdateTask(ctx context.Context, task *SharedTaskV4) error {
    task.UpdatedAt = time.Now()
	query := `
		UPDATE shared_tasks_v4
        SET title = $1, description = $2, status = $3, agent_id = $4, priority = $5, payload = $6, parent_plan_id = $7, dependencies = $8, updated_at = $9
        WHERE id = $10
	`
	_, err := o.db.ExecContext(ctx, query,
		task.Title, task.Description, task.Status, task.AgentID,
		task.Priority, task.Payload, task.ParentPlanID, task.Dependencies, task.UpdatedAt, task.ID,
	)
	return err
}
