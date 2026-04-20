package tasks

import (
	"context"
	"database/sql"
	"errors"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// TaskDecomposition represents a decomposed task in the shared task list.
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
	LockedUntil     sql.NullTime
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

var (
	ErrTaskNotFound = errors.New("task not found")
	ErrTaskClaimed  = errors.New("task already claimed or no tasks available")
)

// TaskDecompositionService manages the lifecycle of shared tasks.
type TaskDecompositionService struct {
	provider db.Provider
	mu       sync.Mutex // fallback for sqlite concurrency handling
}

// NewTaskDecompositionService creates a new service instance.
func NewTaskDecompositionService(provider db.Provider) *TaskDecompositionService {
	return &TaskDecompositionService{
		provider: provider,
	}
}

// CreateTask inserts a new task into the database.
func (s *TaskDecompositionService) CreateTask(ctx context.Context, task *TaskDecomposition) error {
	query := `
		INSERT INTO shared_tasks_decomposition (
			id, organization_id, title, description, status, assigned_agent_id,
			priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
		)
	`
	_, err := s.provider.Exec(ctx, query,
		task.ID, task.OrganizationID, task.Title, task.Description, task.Status, task.AssignedAgentID,
		task.Priority, task.Payload, task.ParentPlanID, task.Dependencies, task.LockedUntil, time.Now(), time.Now(),
	)
	return err
}

// GetTask retrieves a task by its ID.
func (s *TaskDecompositionService) GetTask(ctx context.Context, id string) (*TaskDecomposition, error) {
	query := `
		SELECT
			id, organization_id, title, description, status, assigned_agent_id,
			priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
		FROM shared_tasks_decomposition
		WHERE id = $1
	`
	var task TaskDecomposition
	row := s.provider.QueryRow(ctx, query, id)
	err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID,
		&task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, ErrTaskNotFound
		}
		return nil, err
	}
	return &task, nil
}

// UpdateTask updates an existing task.
func (s *TaskDecompositionService) UpdateTask(ctx context.Context, task *TaskDecomposition) error {
	query := `
		UPDATE shared_tasks_decomposition
		SET
			title = $1, description = $2, status = $3, assigned_agent_id = $4,
			priority = $5, payload = $6, parent_plan_id = $7, dependencies = $8, locked_until = $9, updated_at = $10
		WHERE id = $11
	`
	_, err := s.provider.Exec(ctx, query,
		task.Title, task.Description, task.Status, task.AssignedAgentID,
		task.Priority, task.Payload, task.ParentPlanID, task.Dependencies, task.LockedUntil, time.Now(), task.ID,
	)
	return err
}

// ClaimTask atomicity is handled based on db type.
// For Postgres, it uses FOR UPDATE SKIP LOCKED.
// For SQLite, it uses an application level lock around a simple transaction.
func (s *TaskDecompositionService) ClaimTask(ctx context.Context, orgID string, agentID string) (*TaskDecomposition, error) {
	if s.provider.IsSQLite() {
		return s.claimTaskSQLite(ctx, orgID, agentID)
	}
	return s.claimTaskPostgres(ctx, orgID, agentID)
}

func (s *TaskDecompositionService) claimTaskPostgres(ctx context.Context, orgID string, agentID string) (*TaskDecomposition, error) {
	query := `
		UPDATE shared_tasks_decomposition
		SET status = 'CLAIMED', assigned_agent_id = $1, updated_at = $2
		WHERE id = (
			SELECT id
			FROM shared_tasks_decomposition
			WHERE organization_id = $3 AND status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, organization_id, title, description, status, assigned_agent_id,
			priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
	`
	var task TaskDecomposition
	row := s.provider.QueryRow(ctx, query, agentID, time.Now(), orgID)
	err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID,
		&task.Priority, &task.Payload, &task.ParentPlanID, &task.Dependencies, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, ErrTaskClaimed
		}
		return nil, err
	}
	return &task, nil
}

func (s *TaskDecompositionService) claimTaskSQLite(ctx context.Context, orgID string, agentID string) (*TaskDecomposition, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	// Find pending task
	querySelect := `
		SELECT id
		FROM shared_tasks_decomposition
		WHERE organization_id = $1 AND status = 'PENDING'
		ORDER BY priority ASC, created_at ASC
		LIMIT 1
	`
	var taskID string
	err = tx.QueryRow(ctx, querySelect, orgID).Scan(&taskID)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, ErrTaskClaimed
		}
		return nil, err
	}

	// Update task status to CLAIMED
	queryUpdate := `
		UPDATE shared_tasks_decomposition
		SET status = 'CLAIMED', assigned_agent_id = $1, updated_at = $2
		WHERE id = $3
	`
	_, err = tx.Exec(ctx, queryUpdate, agentID, time.Now(), taskID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return s.GetTask(ctx, taskID)
}

// MarkTaskDone transitions a task to DONE status.
func (s *TaskDecompositionService) MarkTaskDone(ctx context.Context, taskID string) error {
	query := `UPDATE shared_tasks_decomposition SET status = 'DONE', updated_at = $1 WHERE id = $2`
	_, err := s.provider.Exec(ctx, query, time.Now(), taskID)
	return err
}

// MarkTaskFailed transitions a task to FAILED status.
func (s *TaskDecompositionService) MarkTaskFailed(ctx context.Context, taskID string) error {
	query := `UPDATE shared_tasks_decomposition SET status = 'FAILED', updated_at = $1 WHERE id = $2`
	_, err := s.provider.Exec(ctx, query, time.Now(), taskID)
	return err
}
