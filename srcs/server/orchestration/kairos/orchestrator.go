package kairos

import (
	"context"
	"database/sql"
	"errors"
	"time"

	"github.com/google/uuid"
)

// SharedTaskV4 represents the schema for shared_tasks_v4.
type SharedTaskV4 struct {
	ID             string
	OrganizationID string
	Title          string
	Description    *string
	Status         string
	AgentID        *string
	Priority       string
	Payload        *string
	ParentPlanID   *string
	Dependencies   string
	CreatedAt      time.Time
	UpdatedAt      time.Time
}

// SharedTaskOrchestrator provides an interface to the shared_tasks_v4 table.
type SharedTaskOrchestrator struct {
	db *sql.DB
}

// NewSharedTaskOrchestrator creates a new SharedTaskOrchestrator.
func NewSharedTaskOrchestrator(db *sql.DB) *SharedTaskOrchestrator {
	return &SharedTaskOrchestrator{db: db}
}

// CreateTask creates a new task in the database.
func (o *SharedTaskOrchestrator) CreateTask(ctx context.Context, task *SharedTaskV4) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}

	if task.Status == "" {
		task.Status = "PENDING"
	}

	if task.Priority == "" {
		task.Priority = "P2"
	}

	if task.Dependencies == "" {
		task.Dependencies = "[]"
	}

	query := `
		INSERT INTO shared_tasks_v4 (id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`
	_, err := o.db.ExecContext(ctx, query,
		task.ID, task.OrganizationID, task.Title, task.Description, task.Status,
		task.AgentID, task.Priority, task.Payload, task.ParentPlanID, task.Dependencies,
	)

	if err == nil {
		task.CreatedAt = time.Now()
		task.UpdatedAt = time.Now()
	}

	return err
}

// GetTask retrieves a task by its ID.
func (o *SharedTaskOrchestrator) GetTask(ctx context.Context, id string) (*SharedTaskV4, error) {
	query := `
        SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
        FROM shared_tasks_v4
        WHERE id = $1
    `
	row := o.db.QueryRowContext(ctx, query, id)

	task := &SharedTaskV4{}
	err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
		&task.AgentID, &task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies,
		&task.CreatedAt, &task.UpdatedAt,
	)

	if err == sql.ErrNoRows {
		return nil, errors.New("task not found")
	} else if err != nil {
		return nil, err
	}

	return task, nil
}

// UpdateTaskStatus updates the status of a given task.
func (o *SharedTaskOrchestrator) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	query := `UPDATE shared_tasks_v4 SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	_, err := o.db.ExecContext(ctx, query, status, id)
	return err
}
