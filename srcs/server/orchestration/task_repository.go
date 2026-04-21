package orchestration

import (
	"context"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type TaskRepository struct {
	db db.Provider
}

func NewTaskRepository(database db.Provider) *TaskRepository {
	return &TaskRepository{db: database}
}

func (r *TaskRepository) CreateTask(ctx context.Context, task *TaskEntity) error {
	query := `
		INSERT INTO ohc_tasks (id, title, description, status, assigned_agent_id, priority, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`
	_, err := r.db.Exec(ctx, query, task.ID, task.Title, task.Description, task.Status, task.AssignedAgentID, task.Priority, task.CreatedAt, task.UpdatedAt)
	return err
}

func (r *TaskRepository) GetTask(ctx context.Context, id string) (*TaskEntity, error) {
	query := `
		SELECT id, title, description, status, assigned_agent_id, priority, created_at, updated_at
		FROM ohc_tasks
		WHERE id = $1
	`
	task := &TaskEntity{}
	err := r.db.QueryRow(ctx, query, id).Scan(
		&task.ID,
		&task.Title,
		&task.Description,
		&task.Status,
		&task.AssignedAgentID,
		&task.Priority,
		&task.CreatedAt,
		&task.UpdatedAt,
	)
	if err != nil {
		return nil, err
	}
	return task, nil
}

func (r *TaskRepository) ListTasks(ctx context.Context) ([]*TaskEntity, error) {
	query := `
		SELECT id, title, description, status, assigned_agent_id, priority, created_at, updated_at
		FROM ohc_tasks
		ORDER BY created_at DESC
	`
	rows, err := r.db.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*TaskEntity
	for rows.Next() {
		task := &TaskEntity{}
		if err := rows.Scan(
			&task.ID,
			&task.Title,
			&task.Description,
			&task.Status,
			&task.AssignedAgentID,
			&task.Priority,
			&task.CreatedAt,
			&task.UpdatedAt,
		); err != nil {
			return nil, err
		}
		tasks = append(tasks, task)
	}
	return tasks, nil
}

func (r *TaskRepository) ClaimTask(ctx context.Context, taskID string, agentID string) (bool, error) {
	tx, err := r.db.Begin(ctx)
	if err != nil {
		return false, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var claimed bool
	if r.db.IsSQLite() {
		// SQLite logic
		query := `
			UPDATE ohc_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = $2 AND status = 'PENDING'
		`
		result, err := tx.Exec(ctx, query, agentID, taskID)
		if err != nil {
			return false, err
		}
		rowsAffected := result
		claimed = rowsAffected > 0
	} else {
		// Postgres logic
		query := `
			UPDATE ohc_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = NOW()
			WHERE id = (
				SELECT id FROM ohc_tasks WHERE id = $2 AND status = 'PENDING' FOR UPDATE SKIP LOCKED
			)
			RETURNING id
		`
		var claimedID string
		err := tx.QueryRow(ctx, query, agentID, taskID).Scan(&claimedID)
		if err != nil {
			if err.Error() == "no rows in result set" {
				return false, nil
			}
			return false, err
		}
		claimed = claimedID == taskID
	}

	if claimed {
		if err := tx.Commit(ctx); err != nil {
			return false, err
		}
		return true, nil
	}

	return false, nil
}

func (r *TaskRepository) UpdateTaskStatus(ctx context.Context, taskID, status string) error {
	query := `
		UPDATE ohc_tasks
		SET status = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`
	_, err := r.db.Exec(ctx, query, status, taskID)
	return err
}
