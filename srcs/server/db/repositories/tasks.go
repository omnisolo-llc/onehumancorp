package repositories

import (
	"context"
	"database/sql"
	"errors"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/models"
)

var ErrTaskNotFound = errors.New("task not found")

type TaskRepository interface {
	CreateTask(ctx context.Context, task *models.SwarmTask) error
	GetPendingTasks(ctx context.Context) ([]*models.SwarmTask, error)
	ClaimTask(ctx context.Context, taskID, agentID string) error
	CompleteTask(ctx context.Context, taskID string) error
	GetTaskDependencies(ctx context.Context, taskID string) ([]string, error)
	AddDependency(ctx context.Context, taskID, dependsOn string) error
}

type taskRepositoryImpl struct {
	provider db.Provider
}

func NewTaskRepository(provider db.Provider) TaskRepository {
	return &taskRepositoryImpl{provider: provider}
}

func (r *taskRepositoryImpl) CreateTask(ctx context.Context, task *models.SwarmTask) error {
	query := `
		INSERT INTO swarm_tasks (id, title, description, status, priority, agent_id, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`
	if task.ID == "" {
		task.ID = uuid.NewString()
	}
	now := time.Now()
	task.CreatedAt = now
	task.UpdatedAt = now
	if task.Status == "" {
		task.Status = "PENDING"
	}
	if task.Priority == "" {
		task.Priority = "P2"
	}

	_, err := r.provider.Exec(ctx, query, task.ID, task.Title, task.Description, task.Status, task.Priority, task.AgentID, task.CreatedAt, task.UpdatedAt)
	return err
}

func (r *taskRepositoryImpl) GetPendingTasks(ctx context.Context) ([]*models.SwarmTask, error) {
	query := `
		SELECT id, title, description, status, priority, agent_id, created_at, updated_at
		FROM swarm_tasks
		WHERE status = 'PENDING'
		ORDER BY created_at ASC
	`
	rows, err := r.provider.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*models.SwarmTask
	for rows.Next() {
		var t models.SwarmTask
		var desc sql.NullString
		var agentID sql.NullString
		if err := rows.Scan(&t.ID, &t.Title, &desc, &t.Status, &t.Priority, &agentID, &t.CreatedAt, &t.UpdatedAt); err != nil {
			return nil, err
		}
		if desc.Valid {
			t.Description = desc.String
		}
		if agentID.Valid {
			a := agentID.String
			t.AgentID = &a
		}
		tasks = append(tasks, &t)
	}
	return tasks, rows.Err()
}

func (r *taskRepositoryImpl) ClaimTask(ctx context.Context, taskID, agentID string) error {
	tx, err := r.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var query string
	if r.provider.IsSQLite() {
		query = `
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS', agent_id = $1, updated_at = $2
			WHERE id = $3 AND status = 'PENDING'
		`
		var res int64
		res, err = tx.Exec(ctx, query, agentID, time.Now(), taskID)
		if err == nil && res == 0 {
			return ErrTaskNotFound
		}
	} else {
		// Postgres with SKIP LOCKED
		query = `
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS', agent_id = $1, updated_at = $2
			WHERE id = (
				SELECT id FROM swarm_tasks WHERE id = $3 AND status = 'PENDING' FOR UPDATE SKIP LOCKED
			)
		`
		var res int64
		res, err = tx.Exec(ctx, query, agentID, time.Now(), taskID)
		if err == nil && res == 0 {
			return ErrTaskNotFound
		}
	}

	if err != nil {
		return err
	}

	// Record transition
	transitionQuery := `
		INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, triggered_by, transitioned_at)
		VALUES ($1, $2, $3, $4, $5, $6)
	`
	_, err = tx.Exec(ctx, transitionQuery, uuid.NewString(), taskID, "PENDING", "IN_PROGRESS", agentID, time.Now())
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (r *taskRepositoryImpl) CompleteTask(ctx context.Context, taskID string) error {
	tx, err := r.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_tasks
		SET status = 'DONE', updated_at = $1
		WHERE id = $2
	`
	res, err := tx.Exec(ctx, query, time.Now(), taskID)
	if err != nil {
		return err
	}
	if res == 0 {
		return ErrTaskNotFound
	}

	// Record transition
	transitionQuery := `
		INSERT INTO state_machine_transitions (id, task_id, from_state, to_state, triggered_by, transitioned_at)
		VALUES ($1, $2, $3, $4, $5, $6)
	`
	// Assuming it was IN_PROGRESS
	_, err = tx.Exec(ctx, transitionQuery, uuid.NewString(), taskID, "IN_PROGRESS", "DONE", "system", time.Now())
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (r *taskRepositoryImpl) GetTaskDependencies(ctx context.Context, taskID string) ([]string, error) {
	query := `SELECT depends_on_task_id FROM task_dependencies WHERE task_id = $1`
	rows, err := r.provider.Query(ctx, query, taskID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var deps []string
	for rows.Next() {
		var depID string
		if err := rows.Scan(&depID); err != nil {
			return nil, err
		}
		deps = append(deps, depID)
	}
	return deps, rows.Err()
}

func (r *taskRepositoryImpl) AddDependency(ctx context.Context, taskID, dependsOn string) error {
	query := `INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ($1, $2)`
	_, err := r.provider.Exec(ctx, query, taskID, dependsOn)
	return err
}
