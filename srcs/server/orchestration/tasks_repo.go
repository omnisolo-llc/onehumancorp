package orchestration

import (
	"context"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type TasksRepository struct {
	dbProvider db.Provider
	mu         sync.Mutex
}

func NewTasksRepository(dbProvider db.Provider) *TasksRepository {
	return &TasksRepository{
		dbProvider: dbProvider,
	}
}

func (r *TasksRepository) CreateTask(ctx context.Context, task *OrchestrationTask) error {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, `
		INSERT INTO tasks (id, epic_id, title, status, payload, locked_by, locked_at, created_at, updated_at)
		VALUES (\$1, \$2, \$3, \$4, \$5, \$6, \$7, \$8, \$9)
	`, task.ID, task.EpicID, task.Title, task.Status, task.Payload, task.LockedBy, task.LockedAt, task.CreatedAt, task.UpdatedAt)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (r *TasksRepository) UpdateTaskStatus(ctx context.Context, id, status string) error {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, "UPDATE tasks SET status = \$1, updated_at = CURRENT_TIMESTAMP WHERE id = \$2", status, id)
	if err != nil {
		return err
	}

	return tx.Commit(ctx)
}

func (r *TasksRepository) GetNextAvailableTask(ctx context.Context, agentID string) (*OrchestrationTask, error) {
	if r.dbProvider.IsSQLite() {
		return r.getNextAvailableTaskSQLite(ctx, agentID)
	}
	return r.getNextAvailableTaskPostgres(ctx, agentID)
}

func (r *TasksRepository) getNextAvailableTaskSQLite(ctx context.Context, agentID string) (*OrchestrationTask, error) {
	r.mu.Lock()
	defer r.mu.Unlock()

	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task OrchestrationTask
	row := tx.QueryRow(ctx, "SELECT id, epic_id, title, status, payload, locked_by, locked_at, created_at, updated_at FROM tasks WHERE status = 'PENDING' AND locked_by IS NULL LIMIT 1")
	if err := row.Scan(&task.ID, &task.EpicID, &task.Title, &task.Status, &task.Payload, &task.LockedBy, &task.LockedAt, &task.CreatedAt, &task.UpdatedAt); err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	now := time.Now()
	_, err = tx.Exec(ctx, "UPDATE tasks SET status = 'IN_PROGRESS', locked_by = \$1, locked_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = \$2", agentID, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	task.LockedBy = &agentID
	task.LockedAt = &now
	return &task, nil
}

func (r *TasksRepository) getNextAvailableTaskPostgres(ctx context.Context, agentID string) (*OrchestrationTask, error) {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task OrchestrationTask
	row := tx.QueryRow(ctx, "SELECT id, epic_id, title, status, payload, locked_by, locked_at, created_at, updated_at FROM tasks WHERE status = 'PENDING' AND locked_by IS NULL LIMIT 1 FOR UPDATE SKIP LOCKED")
	if err := row.Scan(&task.ID, &task.EpicID, &task.Title, &task.Status, &task.Payload, &task.LockedBy, &task.LockedAt, &task.CreatedAt, &task.UpdatedAt); err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	now := time.Now()
	_, err = tx.Exec(ctx, "UPDATE tasks SET status = 'IN_PROGRESS', locked_by = \$1, locked_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = \$2", agentID, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	task.LockedBy = &agentID
	task.LockedAt = &now
	return &task, nil
}
