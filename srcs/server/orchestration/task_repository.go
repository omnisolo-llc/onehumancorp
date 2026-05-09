package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"sync"
	"time"
)

type TaskRepository struct {
	db      *sql.DB
	isPgSQL bool
	mu      sync.Mutex // Used for SQLite concurrency control
}

func NewTaskRepository(db *sql.DB, isPgSQL bool) *TaskRepository {
	return &TaskRepository{
		db:      db,
		isPgSQL: isPgSQL,
	}
}

func (r *TaskRepository) Create(ctx context.Context, task *Task) error {
	var payloadBytes []byte
	if task.Payload != nil {
		payloadBytes = []byte(*task.Payload)
	}

	if task.Status == "" {
		task.Status = "PENDING"
	}

	query := `
		INSERT INTO tasks (id, epic_id, title, status, payload, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`
	if !r.isPgSQL {
		query = `
			INSERT INTO tasks (id, epic_id, title, status, payload, created_at, updated_at)
			VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
	}

	_, err := r.db.ExecContext(ctx, query, task.ID, task.EpicID, task.Title, task.Status, payloadBytes)
	return err
}

func (r *TaskRepository) UpdateStatus(ctx context.Context, id string, status string) error {
	query := `UPDATE tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	if !r.isPgSQL {
		query = `UPDATE tasks SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?`
	}
	_, err := r.db.ExecContext(ctx, query, status, id)
	return err
}

func (r *TaskRepository) GetNextAvailableTask(ctx context.Context, workerID string) (*Task, error) {
	if !r.isPgSQL {
		r.mu.Lock()
		defer r.mu.Unlock()
	}

	tx, err := r.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var query string
	if r.isPgSQL {
		query = `
			SELECT id, epic_id, title, status, payload, created_at, updated_at, locked_by, locked_at
			FROM tasks
			WHERE status = 'PENDING'
			  AND NOT EXISTS (
				  SELECT 1 FROM task_dependencies td
				  JOIN tasks dep ON dep.id = td.depends_on_task_id
				  WHERE td.task_id = tasks.id AND dep.status != 'COMPLETED'
			  )
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
	} else {
		query = `
			SELECT id, epic_id, title, status, payload, created_at, updated_at, locked_by, locked_at
			FROM tasks
			WHERE status = 'PENDING'
			  AND NOT EXISTS (
				  SELECT 1 FROM task_dependencies td
				  JOIN tasks dep ON dep.id = td.depends_on_task_id
				  WHERE td.task_id = tasks.id AND dep.status != 'COMPLETED'
			  )
			LIMIT 1
		`
	}

	row := tx.QueryRowContext(ctx, query)
	task := &Task{}
	var payloadBytes []byte
	var createdAtStr, updatedAtStr, lockedAtStr sql.NullString
	var epicID sql.NullString
	var lockedBy sql.NullString
	var lockedAtTime sql.NullTime

	var scanErr error
	if r.isPgSQL {
		scanErr = row.Scan(
			&task.ID, &epicID, &task.Title, &task.Status, &payloadBytes,
			&task.CreatedAt, &task.UpdatedAt, &lockedBy, &lockedAtTime,
		)
	} else {
		scanErr = row.Scan(
			&task.ID, &epicID, &task.Title, &task.Status, &payloadBytes,
			&createdAtStr, &updatedAtStr, &lockedBy, &lockedAtStr,
		)
	}

	if scanErr == sql.ErrNoRows {
		return nil, nil // No task available
	} else if scanErr != nil {
		return nil, scanErr
	}

	if !r.isPgSQL {
		if createdAtStr.Valid {
			if t, err := time.Parse(time.DateTime, createdAtStr.String); err == nil {
				task.CreatedAt = t
			} else if t, err := time.Parse(time.RFC3339, createdAtStr.String); err == nil {
				task.CreatedAt = t
			}
		}
		if updatedAtStr.Valid {
			if t, err := time.Parse(time.DateTime, updatedAtStr.String); err == nil {
				task.UpdatedAt = t
			} else if t, err := time.Parse(time.RFC3339, updatedAtStr.String); err == nil {
				task.UpdatedAt = t
			}
		}
		if lockedAtStr.Valid {
			if t, err := time.Parse(time.DateTime, lockedAtStr.String); err == nil {
				task.LockedAt = &t
			} else if t, err := time.Parse(time.RFC3339, lockedAtStr.String); err == nil {
				task.LockedAt = &t
			}
		}
	} else {
		if lockedAtTime.Valid {
			task.LockedAt = &lockedAtTime.Time
		}
	}

	if epicID.Valid {
		task.EpicID = &epicID.String
	}
	if lockedBy.Valid {
		task.LockedBy = &lockedBy.String
	}
	if len(payloadBytes) > 0 {
		raw := json.RawMessage(payloadBytes)
		task.Payload = &raw
	}

	// Lock the task
	now := time.Now()
	task.Status = "IN_PROGRESS"
	task.LockedBy = &workerID
	task.LockedAt = &now

	updateQuery := `
		UPDATE tasks
		SET status = $1, locked_by = $2, locked_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
		WHERE id = $3
	`
	if !r.isPgSQL {
		updateQuery = `
			UPDATE tasks
			SET status = ?, locked_by = ?, locked_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
			WHERE id = ?
		`
	}

	_, err = tx.ExecContext(ctx, updateQuery, task.Status, workerID, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	return task, nil
}
