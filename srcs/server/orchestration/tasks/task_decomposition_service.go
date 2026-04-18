package tasks

import (
	"context"
	"database/sql"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/google/uuid"
)

type TaskDecomposition struct {
	ID              string
	OrganizationID  string
	Title           string
	Description     *string
	Status          string
	AssignedAgentID *string
	Priority        string
	Payload         *string
	ParentPlanID    *string
	Dependencies    string
	LockedUntil     *time.Time
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

type TaskDecompositionService struct {
	provider db.Provider
}

func NewTaskDecompositionService(provider db.Provider) *TaskDecompositionService {
	return &TaskDecompositionService{
		provider: provider,
	}
}

func (s *TaskDecompositionService) Create(ctx context.Context, task TaskDecomposition) (string, error) {
	if task.ID == "" {
		task.ID = uuid.NewString()
	}

	query := `INSERT INTO shared_tasks_decomposition (
		id, organization_id, title, description, status, assigned_agent_id,
		priority, payload, parent_plan_id, dependencies, locked_until
	) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)`

	_, err := s.provider.Exec(ctx, query,
		task.ID, task.OrganizationID, task.Title, task.Description, task.Status, task.AssignedAgentID,
		task.Priority, task.Payload, task.ParentPlanID, task.Dependencies, task.LockedUntil,
	)

	if err != nil {
		return "", err
	}
	return task.ID, nil
}

func (s *TaskDecompositionService) Get(ctx context.Context, id string) (*TaskDecomposition, error) {
	query := `SELECT id, organization_id, title, description, status, assigned_agent_id,
		priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
	FROM shared_tasks_decomposition WHERE id = $1`

	var task TaskDecomposition
	err := s.provider.QueryRow(ctx, query, id).Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID,
		&task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}
	return &task, nil
}

func (s *TaskDecompositionService) UpdateState(ctx context.Context, id string, status string) error {
	query := `UPDATE shared_tasks_decomposition SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	_, err := s.provider.Exec(ctx, query, status, id)
	return err
}

func (s *TaskDecompositionService) Claim(ctx context.Context, organizationID string, agentID string) (*TaskDecomposition, error) {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task TaskDecomposition
	var query string

	if s.provider.IsSQLite() {
		// SQLite emulation for SKIP LOCKED - find one, then update it
		// Need EXCLUSIVE transaction to avoid race conditions. Actually, db.Provider Begin starts a standard Tx.
		// However, we can use the mutex approach if needed, or stick to this. SQLite standalone has lower concurrency.
		query = `SELECT id, organization_id, title, description, status, assigned_agent_id,
			priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		FROM shared_tasks_decomposition
		WHERE status = 'PENDING' AND organization_id = $1 ORDER BY priority ASC, created_at ASC LIMIT 1`

		err = tx.QueryRow(ctx, query, organizationID).Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID,
			&task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, err
		}

		updateQuery := `UPDATE shared_tasks_decomposition SET status = 'CLAIMED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
		if err != nil {
			return nil, err
		}
	} else {
		// PostgreSQL with FOR UPDATE SKIP LOCKED
		query = `SELECT id, organization_id, title, description, status, assigned_agent_id,
			priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		FROM shared_tasks_decomposition
		WHERE status = 'PENDING' AND organization_id = $1 ORDER BY priority ASC, created_at ASC FOR UPDATE SKIP LOCKED LIMIT 1`

		err = tx.QueryRow(ctx, query, organizationID).Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID,
			&task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, err
		}

		updateQuery := `UPDATE shared_tasks_decomposition SET status = 'CLAIMED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
		if err != nil {
			return nil, err
		}
	}

	task.Status = "CLAIMED"
	task.AssignedAgentID = &agentID

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return &task, nil
}
