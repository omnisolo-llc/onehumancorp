package db

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db/models"
)

// SharedTaskRepository defines the interface for managing shared tasks.
type SharedTaskRepository interface {
	AcquireTask(ctx context.Context, organizationID, agentID string) (*TaskRecord, error)
	CreateSharedTask(ctx context.Context, task *models.SharedTask) error
	GetSharedTasks(ctx context.Context, organizationID string) ([]*models.SharedTask, error)
	ClaimSharedTask(ctx context.Context, taskID, agentID string) (bool, error)
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

func (r *sharedTaskRepositoryImpl) CreateSharedTask(ctx context.Context, task *models.SharedTask) error {
	q := `INSERT INTO shared_tasks (id, organization_id, epic_id, parent_plan_id, title, description, priority, status, assigned_agent_id, dependencies, created_at, updated_at)
		  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)`

	now := time.Now().UTC()
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	if task.CreatedAt.IsZero() {
		task.CreatedAt = now
	}
	if task.UpdatedAt.IsZero() {
		task.UpdatedAt = now
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}

	_, err := r.provider.Exec(ctx, q, task.ID, task.OrganizationID, task.EpicID, task.ParentPlanID, task.Title, task.Description, task.Priority, task.Status, task.AssignedAgentID, task.Dependencies, task.CreatedAt, task.UpdatedAt)
	if err != nil {
		return fmt.Errorf("failed to insert shared task: %w", err)
	}

	return nil
}

func (r *sharedTaskRepositoryImpl) GetSharedTasks(ctx context.Context, organizationID string) ([]*models.SharedTask, error) {
	q := `SELECT id, organization_id, epic_id, parent_plan_id, title, description, priority, status, assigned_agent_id, dependencies, created_at, updated_at
		  FROM shared_tasks WHERE organization_id = $1 ORDER BY created_at ASC`

	rows, err := r.provider.Query(ctx, q, organizationID)
	if err != nil {
		return nil, fmt.Errorf("failed to query shared tasks: %w", err)
	}
	defer rows.Close()

	var tasks []*models.SharedTask
	for rows.Next() {
		task := &models.SharedTask{}
		err := rows.Scan(&task.ID, &task.OrganizationID, &task.EpicID, &task.ParentPlanID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.AssignedAgentID, &task.Dependencies, &task.CreatedAt, &task.UpdatedAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan shared task: %w", err)
		}
		tasks = append(tasks, task)
	}
	if err = rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return tasks, nil
}

func (r *sharedTaskRepositoryImpl) ClaimSharedTask(ctx context.Context, taskID, agentID string) (bool, error) {
	isPostgres := !r.provider.IsSQLite()

	if isPostgres {
		tx, err := r.provider.Begin(ctx)
		if err != nil {
			return false, fmt.Errorf("failed to begin tx: %w", err)
		}
		defer tx.Rollback(ctx)

		var currentStatus string
		q := `SELECT status FROM shared_tasks WHERE id = $1 AND status = 'PENDING' FOR UPDATE SKIP LOCKED`
		err = tx.QueryRow(ctx, q, taskID).Scan(&currentStatus)
		if err != nil {
			if err == sql.ErrNoRows || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
				return false, nil
			}
			return false, fmt.Errorf("failed to acquire lock: %w", err)
		}

		updateQ := `UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = $2 WHERE id = $3 AND status = 'PENDING'`
		_, err = tx.Exec(ctx, updateQ, agentID, time.Now().UTC(), taskID)
		if err != nil {
			return false, fmt.Errorf("failed to update shared task status: %w", err)
		}

		if err := tx.Commit(ctx); err != nil {
			return false, fmt.Errorf("failed to commit tx: %w", err)
		}

		return true, nil
	} else {
		// SQLite single UPDATE ... RETURNING
		q := `UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = $2 WHERE id = $3 AND status = 'PENDING' RETURNING id`
		row := r.provider.QueryRow(ctx, q, agentID, time.Now().UTC(), taskID)
		var returnedID string
		err := row.Scan(&returnedID)
		if err != nil {
			if err == sql.ErrNoRows || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
				return false, nil
			}
			return false, fmt.Errorf("failed to claim task in sqlite: %w", err)
		}
		return returnedID != "", nil
	}
}
