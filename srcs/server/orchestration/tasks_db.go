package orchestration

import (
	"context"
	"database/sql"
	"errors"
)

type SharedTask struct {
	ID             string
	OrganizationID string
	Title          string
	Status         string
	Dependencies   []byte
}

type TaskDB struct {
	db     *sql.DB
	isPg   bool
}

func NewTaskDB(db *sql.DB, isPg bool) *TaskDB {
	return &TaskDB{db: db, isPg: isPg}
}

func (t *TaskDB) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	tx, err := t.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var task SharedTask
	var query string

	if t.isPg {
		query = `SELECT id, organization_id, title, status, dependencies FROM shared_tasks WHERE status = 'PENDING' LIMIT 1 FOR UPDATE SKIP LOCKED`
	} else {
		query = `SELECT id, organization_id, title, status, dependencies FROM shared_tasks WHERE status = 'PENDING' LIMIT 1`
	}

	err = tx.QueryRowContext(ctx, query).Scan(&task.ID, &task.OrganizationID, &task.Title, &task.Status, &task.Dependencies)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, err
	}

	// Update the task status to IN_PROGRESS
	updateQuery := `UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = $1`
	if !t.isPg {
		updateQuery = `UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = ?`
	}

	_, err = tx.ExecContext(ctx, updateQuery, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	return &task, nil
}
