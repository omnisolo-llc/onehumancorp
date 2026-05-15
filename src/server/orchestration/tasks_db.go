package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"sync"
)

type SharedTask struct {
	ID             string
	OrganizationID string
	Title          string
	Status         string
	Dependencies   []string
}

type TaskDecomposerEngine struct {
	db       *sql.DB
	isSQLite bool
	mu       sync.Mutex
}

func NewTaskDecomposerEngine(db *sql.DB, isSQLite bool) *TaskDecomposerEngine {
	return &TaskDecomposerEngine{
		db:       db,
		isSQLite: isSQLite,
	}
}

func (e *TaskDecomposerEngine) ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTask, error) {
	if e.isSQLite {
		e.mu.Lock()
		defer e.mu.Unlock()

		tx, err := e.db.BeginTx(ctx, nil)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback()

		row := tx.QueryRowContext(ctx, "SELECT id, title, dependencies FROM shared_tasks WHERE status = 'PENDING' AND organization_id = ? LIMIT 1", organizationID)
		var task SharedTask
		var depsJSON string
		if err := row.Scan(&task.ID, &task.Title, &depsJSON); err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, err
		}

		if err := json.Unmarshal([]byte(depsJSON), &task.Dependencies); err != nil {
			return nil, err
		}

		allDepsDone := true
		for _, depID := range task.Dependencies {
			var status string
			err := tx.QueryRowContext(ctx, "SELECT status FROM shared_tasks WHERE id = ?", depID).Scan(&status)
			if err != nil || status != "DONE" {
				allDepsDone = false
				break
			}
		}

		if !allDepsDone {
			return nil, nil
		}

		_, err = tx.ExecContext(ctx, "UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = ?", task.ID)
		if err != nil {
			return nil, err
		}

		if err := tx.Commit(); err != nil {
			return nil, err
		}

		task.OrganizationID = organizationID
		task.Status = "IN_PROGRESS"
		return &task, nil

	} else {
		tx, err := e.db.BeginTx(ctx, nil)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback()

		row := tx.QueryRowContext(ctx, "SELECT id, title, dependencies FROM shared_tasks WHERE status = 'PENDING' AND organization_id = $1 FOR UPDATE SKIP LOCKED LIMIT 1", organizationID)
		var task SharedTask
		var depsJSON string
		if err := row.Scan(&task.ID, &task.Title, &depsJSON); err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, err
		}

		if err := json.Unmarshal([]byte(depsJSON), &task.Dependencies); err != nil {
			return nil, err
		}

		allDepsDone := true
		for _, depID := range task.Dependencies {
			var status string
			err := tx.QueryRowContext(ctx, "SELECT status FROM shared_tasks WHERE id = $1", depID).Scan(&status)
			if err != nil || status != "DONE" {
				allDepsDone = false
				break
			}
		}

		if !allDepsDone {
			return nil, nil
		}

		_, err = tx.ExecContext(ctx, "UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = $1", task.ID)
		if err != nil {
			return nil, err
		}

		if err := tx.Commit(); err != nil {
			return nil, err
		}

		task.OrganizationID = organizationID
		task.Status = "IN_PROGRESS"
		return &task, nil
	}
}
