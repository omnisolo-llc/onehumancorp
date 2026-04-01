package scheduler

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"
)

// SqliteTaskRepository implements TaskRepository backed by SQLite.
// SQLite doesn't support FOR UPDATE SKIP LOCKED natively in the same way,
// but since SQLite is used for Standalone mode (single user/process),
// a simple transaction with immediate locking is sufficient.
type SqliteTaskRepository struct {
	db *sql.DB
}

// NewSqliteTaskRepository creates a SQLite-backed task repository.
func NewSqliteTaskRepository(db *sql.DB) *SqliteTaskRepository {
	return &SqliteTaskRepository{db: db}
}

func (r *SqliteTaskRepository) Create(ctx context.Context, task Task) error {
	payload, _ := json.Marshal(task.Payload)
	var scheduleAt, nextRunAt *string
	if task.Schedule.At != nil {
		s := task.Schedule.At.Format("2006-01-02 15:04:05")
		scheduleAt = &s
	}
	if task.NextRunAt != nil {
		s := task.NextRunAt.Format("2006-01-02 15:04:05")
		nextRunAt = &s
	}

	_, err := r.db.ExecContext(ctx, `
		INSERT INTO scheduled_tasks (id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, next_run_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, ?)`,
		task.ID, task.OrganizationID, task.AgentID, task.Name,
		string(task.Schedule.Type), scheduleAt, task.Schedule.IntervalS, task.Schedule.Expression,
		string(task.Status), string(payload), nextRunAt,
	)
	if err != nil {
		return fmt.Errorf("sqlite: create task: %w", err)
	}
	return nil
}

func (r *SqliteTaskRepository) Get(ctx context.Context, id string) (Task, error) {
	task := Task{}
	var schedType, status string
	var payload string
	var scheduleAt, createdAt, lastRunAt, nextRunAt *string

	err := r.db.QueryRowContext(ctx, `
		SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
		FROM scheduled_tasks WHERE id = ?`, id).Scan(
		&task.ID, &task.OrganizationID, &task.AgentID, &task.Name,
		&schedType, &scheduleAt, &task.Schedule.IntervalS, &task.Schedule.Expression,
		&status, &payload, &createdAt, &lastRunAt, &nextRunAt,
	)
	if err != nil {
		return Task{}, fmt.Errorf("sqlite: get task: %w", err)
	}

	task.Schedule.Type = ScheduleType(schedType)
	task.Status = TaskStatus(status)
	task.Payload = json.RawMessage(payload)

	if scheduleAt != nil {
		if t, err := time.Parse("2006-01-02 15:04:05", *scheduleAt); err == nil {
			task.Schedule.At = &t
		}
	}
	if createdAt != nil {
		if t, err := time.Parse("2006-01-02 15:04:05", *createdAt); err == nil {
			task.CreatedAt = t
		}
	}
	if lastRunAt != nil {
		if t, err := time.Parse("2006-01-02 15:04:05", *lastRunAt); err == nil {
			task.LastRunAt = &t
		}
	}
	if nextRunAt != nil {
		if t, err := time.Parse("2006-01-02 15:04:05", *nextRunAt); err == nil {
			task.NextRunAt = &t
		}
	}

	return task, nil
}

func (r *SqliteTaskRepository) ListForOrg(ctx context.Context, orgID string) ([]Task, error) {
	rows, err := r.db.QueryContext(ctx, `
		SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
		FROM scheduled_tasks WHERE organization_id = ? ORDER BY created_at`, orgID)
	if err != nil {
		return nil, fmt.Errorf("sqlite: list tasks: %w", err)
	}
	defer rows.Close()

	var tasks []Task
	for rows.Next() {
		var t Task
		var schedType, status string
		var payload string
		var scheduleAt, createdAt, lastRunAt, nextRunAt *string

		if err := rows.Scan(
			&t.ID, &t.OrganizationID, &t.AgentID, &t.Name,
			&schedType, &scheduleAt, &t.Schedule.IntervalS, &t.Schedule.Expression,
			&status, &payload, &createdAt, &lastRunAt, &nextRunAt,
		); err != nil {
			return nil, fmt.Errorf("sqlite: scan task: %w", err)
		}

		t.Schedule.Type = ScheduleType(schedType)
		t.Status = TaskStatus(status)
		t.Payload = json.RawMessage(payload)

		if scheduleAt != nil {
			if tm, err := time.Parse("2006-01-02 15:04:05", *scheduleAt); err == nil {
				t.Schedule.At = &tm
			}
		}
		if createdAt != nil {
			if tm, err := time.Parse("2006-01-02 15:04:05", *createdAt); err == nil {
				t.CreatedAt = tm
			}
		}
		if lastRunAt != nil {
			if tm, err := time.Parse("2006-01-02 15:04:05", *lastRunAt); err == nil {
				t.LastRunAt = &tm
			}
		}
		if nextRunAt != nil {
			if tm, err := time.Parse("2006-01-02 15:04:05", *nextRunAt); err == nil {
				t.NextRunAt = &tm
			}
		}
		tasks = append(tasks, t)
	}
	return tasks, nil
}

func (r *SqliteTaskRepository) PollDue(ctx context.Context) ([]Task, error) {
	tx, err := r.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("sqlite: begin poll tx: %w", err)
	}
	defer func() { _ = tx.Rollback() }()

	rows, err := tx.QueryContext(ctx, `
		SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
		FROM scheduled_tasks
		WHERE status = 'pending' AND next_run_at <= CURRENT_TIMESTAMP`)
	if err != nil {
		return nil, fmt.Errorf("sqlite: poll due: %w", err)
	}

	var tasks []Task
	for rows.Next() {
		var t Task
		var schedType, status string
		var payload string
		var scheduleAt, createdAt, lastRunAt, nextRunAt *string

		if err := rows.Scan(
			&t.ID, &t.OrganizationID, &t.AgentID, &t.Name,
			&schedType, &scheduleAt, &t.Schedule.IntervalS, &t.Schedule.Expression,
			&status, &payload, &createdAt, &lastRunAt, &nextRunAt,
		); err != nil {
			rows.Close()
			return nil, fmt.Errorf("sqlite: scan due task: %w", err)
		}

		t.Schedule.Type = ScheduleType(schedType)
		t.Status = TaskStatus(status)
		t.Payload = json.RawMessage(payload)

		if scheduleAt != nil {
			if tm, err := time.Parse("2006-01-02 15:04:05", *scheduleAt); err == nil {
				t.Schedule.At = &tm
			}
		}
		if createdAt != nil {
			if tm, err := time.Parse("2006-01-02 15:04:05", *createdAt); err == nil {
				t.CreatedAt = tm
			}
		}
		if lastRunAt != nil {
			if tm, err := time.Parse("2006-01-02 15:04:05", *lastRunAt); err == nil {
				t.LastRunAt = &tm
			}
		}
		if nextRunAt != nil {
			if tm, err := time.Parse("2006-01-02 15:04:05", *nextRunAt); err == nil {
				t.NextRunAt = &tm
			}
		}

		tasks = append(tasks, t)
	}
	rows.Close()

	now := time.Now().UTC().Format("2006-01-02 15:04:05")
	for _, t := range tasks {
		if _, err := tx.ExecContext(ctx, "UPDATE scheduled_tasks SET status='running', last_run_at=? WHERE id=?", now, t.ID); err != nil {
			return nil, fmt.Errorf("sqlite: mark running: %w", err)
		}
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("sqlite: commit poll: %w", err)
	}
	return tasks, nil
}

func (r *SqliteTaskRepository) UpdateStatus(ctx context.Context, id string, status TaskStatus, reschedule bool) error {
	if reschedule {
		_, err := r.db.ExecContext(ctx, `
			UPDATE scheduled_tasks
			SET status = 'pending', next_run_at = datetime('now', '+' || interval_s || ' seconds')
			WHERE id = ?`, id)
		return err
	}
	_, err := r.db.ExecContext(ctx, "UPDATE scheduled_tasks SET status = ? WHERE id = ?", string(status), id)
	return err
}

func (r *SqliteTaskRepository) Cancel(ctx context.Context, id string) error {
	_, err := r.db.ExecContext(ctx, "UPDATE scheduled_tasks SET status = 'cancelled' WHERE id = ?", id)
	return err
}
