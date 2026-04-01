package scheduler

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SqliteTaskRepository implements TaskRepository backed by SQLite.
type SqliteTaskRepository struct {
	pool db.Provider
}

// NewSqliteTaskRepository creates a SQLite-backed task repository.
func NewSqliteTaskRepository(pool db.Provider) *SqliteTaskRepository {
	return &SqliteTaskRepository{pool: pool}
}

func (r *SqliteTaskRepository) Create(ctx context.Context, task Task) error {
	payload, _ := json.Marshal(task.Payload)
	_, err := r.pool.Exec(ctx, `
		INSERT INTO scheduled_tasks (id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, next_run_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		task.ID, task.OrganizationID, task.AgentID, task.Name,
		string(task.Schedule.Type), task.Schedule.At, task.Schedule.IntervalS, task.Schedule.Expression,
		string(task.Status), string(payload), task.CreatedAt, task.NextRunAt,
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
	err := r.pool.QueryRow(ctx, `
		SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
		FROM scheduled_tasks WHERE id = ?`, id).Scan(
		&task.ID, &task.OrganizationID, &task.AgentID, &task.Name,
		&schedType, &task.Schedule.At, &task.Schedule.IntervalS, &task.Schedule.Expression,
		&status, &payload, &task.CreatedAt, &task.LastRunAt, &task.NextRunAt,
	)
	if err != nil {
		return Task{}, fmt.Errorf("sqlite: get task: %w", err)
	}
	task.Schedule.Type = ScheduleType(schedType)
	task.Status = TaskStatus(status)
	task.Payload = json.RawMessage(payload)
	return task, nil
}

func (r *SqliteTaskRepository) ListForOrg(ctx context.Context, orgID string) ([]Task, error) {
	rows, err := r.pool.Query(ctx, `
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
		if err := rows.Scan(
			&t.ID, &t.OrganizationID, &t.AgentID, &t.Name,
			&schedType, &t.Schedule.At, &t.Schedule.IntervalS, &t.Schedule.Expression,
			&status, &payload, &t.CreatedAt, &t.LastRunAt, &t.NextRunAt,
		); err != nil {
			return nil, fmt.Errorf("sqlite: scan task: %w", err)
		}
		t.Schedule.Type = ScheduleType(schedType)
		t.Status = TaskStatus(status)
		t.Payload = json.RawMessage(payload)
		tasks = append(tasks, t)
	}
	return tasks, nil
}

func (r *SqliteTaskRepository) PollDue(ctx context.Context) ([]Task, error) {
	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("sqlite: begin poll tx: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	query := `
		SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
		FROM scheduled_tasks
		WHERE status = 'pending' AND next_run_at <= CURRENT_TIMESTAMP`

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("sqlite: poll due: %w", err)
	}

	var tasks []Task
	now := time.Now().UTC()
	for rows.Next() {
		var t Task
		var schedType, status string
		var payload string
		if err := rows.Scan(
			&t.ID, &t.OrganizationID, &t.AgentID, &t.Name,
			&schedType, &t.Schedule.At, &t.Schedule.IntervalS, &t.Schedule.Expression,
			&status, &payload, &t.CreatedAt, &t.LastRunAt, &t.NextRunAt,
		); err != nil {
			rows.Close()
			return nil, fmt.Errorf("sqlite: scan due task: %w", err)
		}
		t.Schedule.Type = ScheduleType(schedType)
		t.Status = TaskStatus(status)
		t.Payload = json.RawMessage(payload)
		tasks = append(tasks, t)
	}
	rows.Close()

	for _, t := range tasks {
		if _, err := tx.Exec(ctx, "UPDATE scheduled_tasks SET status='running', last_run_at=? WHERE id=?", now, t.ID); err != nil {
			return nil, fmt.Errorf("sqlite: mark running: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("sqlite: commit poll: %w", err)
	}
	return tasks, nil
}

func (r *SqliteTaskRepository) UpdateStatus(ctx context.Context, id string, status TaskStatus, reschedule bool) error {
	if reschedule {
		_, err := r.pool.Exec(ctx, `
			UPDATE scheduled_tasks
			SET status = 'pending', next_run_at = datetime(CURRENT_TIMESTAMP, '+' || interval_s || ' seconds')
			WHERE id = ?`, id)
		return err
	}
	_, err := r.pool.Exec(ctx, "UPDATE scheduled_tasks SET status = ? WHERE id = ?", string(status), id)
	return err
}

func (r *SqliteTaskRepository) Cancel(ctx context.Context, id string) error {
	_, err := r.pool.Exec(ctx, "UPDATE scheduled_tasks SET status = 'cancelled' WHERE id = ?", id)
	return err
}
