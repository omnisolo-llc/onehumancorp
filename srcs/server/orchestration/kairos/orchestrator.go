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

// GetTask retrieves a task by its ID and OrganizationID.
func (o *SharedTaskOrchestrator) GetTask(ctx context.Context, id string, organizationID string) (*SharedTaskV4, error) {
	query := `
        SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
        FROM shared_tasks_v4
        WHERE id = $1 AND organization_id = $2
    `
	row := o.db.QueryRowContext(ctx, query, id, organizationID)

	task := &SharedTaskV4{}
    var desc, agent, payload, parent sql.NullString
	err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Status,
		&agent, &task.Priority, &payload, &parent, &task.Dependencies,
		&task.CreatedAt, &task.UpdatedAt,
	)

	if err == sql.ErrNoRows {
		return nil, errors.New("task not found")
	} else if err != nil {
		return nil, err
	}

    if desc.Valid {
        task.Description = &desc.String
    }
    if agent.Valid {
        task.AgentID = &agent.String
    }
    if payload.Valid {
        task.Payload = &payload.String
    }
    if parent.Valid {
        task.ParentPlanID = &parent.String
    }

	return task, nil
}

// UpdateTaskStatus updates the status of a given task.
func (o *SharedTaskOrchestrator) UpdateTaskStatus(ctx context.Context, id string, organizationID string, status string) error {
	query := `UPDATE shared_tasks_v4 SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND organization_id = $3`
	_, err := o.db.ExecContext(ctx, query, status, id, organizationID)
	return err
}
