package repositories

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/models"
)

type TaskRepository interface {
	CreateTask(ctx context.Context, task *models.SwarmTask) error
	GetPendingTasks(ctx context.Context) ([]*models.SwarmTask, error)
	ClaimTask(ctx context.Context, taskID string, agentID string) (bool, error)
	CompleteTask(ctx context.Context, taskID string) error
	GetTaskDependencies(ctx context.Context, taskID string) ([]string, error)
}

type taskRepositoryImpl struct {
	dbProvider db.Provider
}

func NewTaskRepository(dbProvider db.Provider) TaskRepository {
	return &taskRepositoryImpl{
		dbProvider: dbProvider,
	}
}

func (r *taskRepositoryImpl) CreateTask(ctx context.Context, task *models.SwarmTask) error {
	q := `INSERT INTO swarm_tasks (id, title, description, status, priority, agent_id, created_at, updated_at)
		  VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`

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
	if task.Priority == "" {
		task.Priority = "P0"
	}

	_, err := r.dbProvider.Exec(ctx, q, task.ID, task.Title, task.Description, task.Status, task.Priority, task.AgentID, task.CreatedAt, task.UpdatedAt)
	if err != nil {
		return fmt.Errorf("failed to insert task: %w", err)
	}

	return nil
}

func (r *taskRepositoryImpl) GetPendingTasks(ctx context.Context) ([]*models.SwarmTask, error) {
	q := `SELECT id, title, description, status, priority, agent_id, created_at, updated_at
		  FROM swarm_tasks WHERE status = 'PENDING' ORDER BY priority DESC, created_at ASC`

	rows, err := r.dbProvider.Query(ctx, q)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending tasks: %w", err)
	}
	defer rows.Close()

	var tasks []*models.SwarmTask
	for rows.Next() {
		task := &models.SwarmTask{}
		err := rows.Scan(&task.ID, &task.Title, &task.Description, &task.Status, &task.Priority, &task.AgentID, &task.CreatedAt, &task.UpdatedAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan task: %w", err)
		}
		tasks = append(tasks, task)
	}
	if err = rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return tasks, nil
}

func (r *taskRepositoryImpl) ClaimTask(ctx context.Context, taskID string, agentID string) (bool, error) {
	// Determine if we are using PostgreSQL to append the lock clause
	isPostgres := !r.dbProvider.IsSQLite()

	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return false, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var currentStatus string

	// Prepare select query with optional locking
	selectQ := `SELECT status FROM swarm_tasks WHERE id = $1 AND status = 'PENDING'`
	if isPostgres {
		selectQ += ` FOR UPDATE SKIP LOCKED`
	}

	err = tx.QueryRow(ctx, selectQ, taskID).Scan(&currentStatus)
	if err != nil {
		if err == sql.ErrNoRows {
			// Task not found or not pending (or locked by someone else in PG)
			return false, nil
		}
		// In pgx, err isn't exactly sql.ErrNoRows for no rows, but its error string often contains "no rows"
		// Better to just check if it's "no rows" string
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return false, nil
		}

		return false, fmt.Errorf("failed to scan task status: %w", err)
	}

	if currentStatus != "PENDING" {
		return false, nil
	}

	updateQ := `UPDATE swarm_tasks SET status = 'IN_PROGRESS', agent_id = $1, updated_at = $2 WHERE id = $3 AND status = 'PENDING'`
	affected, err := tx.Exec(ctx, updateQ, agentID, time.Now().UTC(), taskID)
	if err != nil {
		return false, fmt.Errorf("failed to update task status: %w", err)
	}

	if affected == 0 {
		return false, nil
	}

	if err := tx.Commit(ctx); err != nil {
		return false, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return true, nil
}

func (r *taskRepositoryImpl) CompleteTask(ctx context.Context, taskID string) error {
	q := `UPDATE swarm_tasks SET status = 'DONE', updated_at = $1 WHERE id = $2`
	_, err := r.dbProvider.Exec(ctx, q, time.Now().UTC(), taskID)
	if err != nil {
		return fmt.Errorf("failed to complete task: %w", err)
	}

	return nil
}

func (r *taskRepositoryImpl) GetTaskDependencies(ctx context.Context, taskID string) ([]string, error) {
	q := `SELECT depends_on_task_id FROM task_dependencies WHERE task_id = $1`
	rows, err := r.dbProvider.Query(ctx, q, taskID)
	if err != nil {
		return nil, fmt.Errorf("failed to query task dependencies: %w", err)
	}
	defer rows.Close()

	var deps []string
	for rows.Next() {
		var depID string
		if err := rows.Scan(&depID); err != nil {
			return nil, fmt.Errorf("failed to scan dep id: %w", err)
		}
		deps = append(deps, depID)
	}
	if err = rows.Err(); err != nil {
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return deps, nil
}
