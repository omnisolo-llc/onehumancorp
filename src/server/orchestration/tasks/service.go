package tasks

import (
	"context"

	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

type TaskDecompositionService struct {
	provider db.Provider
}

func NewTaskDecompositionService(provider db.Provider) *TaskDecompositionService {
	return &TaskDecompositionService{
		provider: provider,
	}
}

func (s *TaskDecompositionService) CreateTask(ctx context.Context, task *SharedTaskDecomposition) error {
	query := `
		INSERT INTO shared_tasks_decomposition (
			id, organization_id, title, description, status, assigned_agent_id,
			priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
		)
	`
	_, err := s.provider.Exec(ctx, query,
		task.ID, task.OrganizationID, task.Title, task.Description, task.Status,
		task.AssignedAgentID, task.Priority, string(task.Payload), task.ParentPlanID,
		string(task.Dependencies), task.LockedUntil, task.CreatedAt, task.UpdatedAt,
	)
	return err
}

func (s *TaskDecompositionService) ClaimTask(ctx context.Context, organizationID, agentID string) (*SharedTaskDecomposition, error) {
	if s.provider.IsSQLite() {
		return s.claimTaskSQLite(ctx, organizationID, agentID)
	}
	return s.claimTaskPostgres(ctx, organizationID, agentID)
}

func (s *TaskDecompositionService) claimTaskPostgres(ctx context.Context, organizationID, agentID string) (*SharedTaskDecomposition, error) {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := `
		SELECT id, organization_id, title, description, status, assigned_agent_id,
		       priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		FROM shared_tasks_decomposition
		WHERE organization_id = $1
		  AND status = 'PENDING'
		  AND (locked_until IS NULL OR locked_until < NOW())
		ORDER BY
			CASE WHEN priority = 'P0' THEN 1
			     WHEN priority = 'P1' THEN 2
			     WHEN priority = 'P2' THEN 3
			     ELSE 4 END,
			created_at ASC
		LIMIT 1
		FOR UPDATE SKIP LOCKED
	`

	row := tx.QueryRow(ctx, query, organizationID)

	task := &SharedTaskDecomposition{}
	var assignedAgentID, parentPlanID *string
	var payload, dependencies []byte
	var createdAt, updatedAt time.Time

	err = row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
		&assignedAgentID, &task.Priority, &payload, &parentPlanID, &dependencies,
		&task.LockedUntil, &createdAt, &updatedAt,
	)
	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil // No task available
		}
		return nil, err
	}

	task.AssignedAgentID = assignedAgentID
	task.ParentPlanID = parentPlanID
	task.Payload = payload
	task.Dependencies = dependencies
	task.CreatedAt = createdAt
	task.UpdatedAt = updatedAt

	// Update the task to CLAIMED
	now := time.Now().UTC()
	lockTime := now.Add(5 * time.Minute)
	updateQuery := `
		UPDATE shared_tasks_decomposition
		SET status = 'CLAIMED', assigned_agent_id = $1, locked_until = $2, updated_at = $3
		WHERE id = $4
	`
	_, err = tx.Exec(ctx, updateQuery, agentID, lockTime, now, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = TaskStatusClaimed
	task.AssignedAgentID = &agentID
	task.LockedUntil = &lockTime
	task.UpdatedAt = now

	return task, nil
}

func (s *TaskDecompositionService) claimTaskSQLite(ctx context.Context, organizationID, agentID string) (*SharedTaskDecomposition, error) {
	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := `
		SELECT id, organization_id, title, description, status, assigned_agent_id,
		       priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		FROM shared_tasks_decomposition
		WHERE organization_id = $1
		  AND status = 'PENDING'
		  AND (locked_until IS NULL OR locked_until < datetime('now'))
		ORDER BY
			CASE WHEN priority = 'P0' THEN 1
			     WHEN priority = 'P1' THEN 2
			     WHEN priority = 'P2' THEN 3
			     ELSE 4 END,
			created_at ASC
		LIMIT 1
	`

	row := tx.QueryRow(ctx, query, organizationID)

	task := &SharedTaskDecomposition{}
	var assignedAgentID, parentPlanID *string
	var payload, dependencies []byte
	var lockedUntil *time.Time
	var createdAt, updatedAt string

	err = row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
		&assignedAgentID, &task.Priority, &payload, &parentPlanID, &dependencies,
		&lockedUntil, &createdAt, &updatedAt,
	)
	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil // No task available
		}
		return nil, err
	}

	// Parse SQLite timestamps
	task.CreatedAt, _ = time.Parse(time.RFC3339Nano, createdAt)
	if task.CreatedAt.IsZero() {
		task.CreatedAt, _ = time.Parse("2006-01-02 15:04:05-07:00", createdAt)
	}
	task.UpdatedAt, _ = time.Parse(time.RFC3339Nano, updatedAt)
	if task.UpdatedAt.IsZero() {
		task.UpdatedAt, _ = time.Parse("2006-01-02 15:04:05-07:00", updatedAt)
	}

	task.AssignedAgentID = assignedAgentID
	task.ParentPlanID = parentPlanID
	task.Payload = payload
	task.Dependencies = dependencies
	task.LockedUntil = lockedUntil

	// Update the task to CLAIMED
	now := time.Now().UTC()
	lockTime := now.Add(5 * time.Minute)
	updateQuery := `
		UPDATE shared_tasks_decomposition
		SET status = 'CLAIMED', assigned_agent_id = $1, locked_until = $2, updated_at = $3
		WHERE id = $4
	`
	_, err = tx.Exec(ctx, updateQuery, agentID, lockTime, now, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = TaskStatusClaimed
	task.AssignedAgentID = &agentID
	task.LockedUntil = &lockTime
	task.UpdatedAt = now

	return task, nil
}

func (s *TaskDecompositionService) UpdateTaskStatus(ctx context.Context, id string, status TaskStatus) error {
	now := time.Now().UTC()
	query := `
		UPDATE shared_tasks_decomposition
		SET status = $1, updated_at = $2
		WHERE id = $3
	`
	_, err := s.provider.Exec(ctx, query, string(status), now, id)
	return err
}

func (s *TaskDecompositionService) GetTask(ctx context.Context, id string) (*SharedTaskDecomposition, error) {
	query := `
		SELECT id, organization_id, title, description, status, assigned_agent_id,
		       priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		FROM shared_tasks_decomposition
		WHERE id = $1
	`
	row := s.provider.QueryRow(ctx, query, id)

	task := &SharedTaskDecomposition{}
	var assignedAgentID, parentPlanID *string
	var payload, dependencies []byte
	var createdAtStr, updatedAtStr string

	var err error
	if s.provider.IsSQLite() {
		err = row.Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
			&assignedAgentID, &task.Priority, &payload, &parentPlanID, &dependencies,
			&task.LockedUntil, &createdAtStr, &updatedAtStr,
		)
		if err == nil {
			task.CreatedAt, _ = time.Parse(time.RFC3339Nano, createdAtStr)
			if task.CreatedAt.IsZero() {
				task.CreatedAt, _ = time.Parse("2006-01-02 15:04:05-07:00", createdAtStr)
			}
			task.UpdatedAt, _ = time.Parse(time.RFC3339Nano, updatedAtStr)
			if task.UpdatedAt.IsZero() {
				task.UpdatedAt, _ = time.Parse("2006-01-02 15:04:05-07:00", updatedAtStr)
			}
		}
	} else {
		err = row.Scan(
			&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
			&assignedAgentID, &task.Priority, &payload, &parentPlanID, &dependencies,
			&task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if err != nil {
		return nil, err
	}

	task.AssignedAgentID = assignedAgentID
	task.ParentPlanID = parentPlanID
	task.Payload = payload
	task.Dependencies = dependencies

	return task, nil
}

func (s *TaskDecompositionService) FailTask(ctx context.Context, id string) error {
	return s.UpdateTaskStatus(ctx, id, TaskStatusFailed)
}

func (s *TaskDecompositionService) CompleteTask(ctx context.Context, id string) error {
	return s.UpdateTaskStatus(ctx, id, TaskStatusDone)
}
