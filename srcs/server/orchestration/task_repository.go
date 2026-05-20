package orchestration

import (
	"context"
	"database/sql"
	"time"

	"github.com/google/uuid"
)

// TaskRepository handles database operations for tasks.
type TaskRepository struct {
	db *sql.DB
}

// NewTaskRepository creates a new TaskRepository.
func NewTaskRepository(db *sql.DB) *TaskRepository {
	return &TaskRepository{db: db}
}

// CreateTask inserts a new task into the database.
func (r *TaskRepository) CreateTask(ctx context.Context, task *Task) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}
	now := time.Now()
	task.CreatedAt = now
	task.UpdatedAt = now

	query := `
		INSERT INTO ohc_tasks (id, title, description, status, assigned_agent_id, priority, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`
	_, err := r.db.ExecContext(ctx, query,
		task.ID, task.Title, task.Description, task.Status, task.AssignedAgentID, task.Priority, task.CreatedAt, task.UpdatedAt)
	return err
}

// GetTask retrieves a task by ID.
func (r *TaskRepository) GetTask(ctx context.Context, id string) (*Task, error) {
	query := `
		SELECT id, title, description, status, assigned_agent_id, priority, created_at, updated_at
		FROM ohc_tasks WHERE id = $1
	`
	row := r.db.QueryRowContext(ctx, query, id)

	var task Task
	err := row.Scan(
		&task.ID, &task.Title, &task.Description, &task.Status,
		&task.AssignedAgentID, &task.Priority, &task.CreatedAt, &task.UpdatedAt,
	)
	if err != nil {
		return nil, err
	}
	return &task, nil
}

// ListTasks retrieves all tasks.
func (r *TaskRepository) ListTasks(ctx context.Context) ([]*Task, error) {
	query := `
		SELECT id, title, description, status, assigned_agent_id, priority, created_at, updated_at
		FROM ohc_tasks ORDER BY created_at DESC
	`
	rows, err := r.db.QueryContext(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*Task
	for rows.Next() {
		var task Task
		err := rows.Scan(
			&task.ID, &task.Title, &task.Description, &task.Status,
			&task.AssignedAgentID, &task.Priority, &task.CreatedAt, &task.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
		tasks = append(tasks, &task)
	}
	return tasks, nil
}

// ClaimTask attempts to atomically claim a pending task for an agent.
func (r *TaskRepository) ClaimTask(ctx context.Context, taskID string, agentID string) (bool, error) {
	// Atomic claim operation using a distributed lock backed by the database
	query := `
		UPDATE ohc_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'PENDING'
	`
	result, err := r.db.ExecContext(ctx, query, agentID, taskID)
	if err != nil {
		return false, err
	}

	rowsAffected, err := result.RowsAffected()
	if err != nil {
		return false, err
	}

	return rowsAffected > 0, nil
}
