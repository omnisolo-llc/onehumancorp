package scheduler

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// PgTaskRepository implements TaskRepository backed by PostgreSQL or SQLite.
type PgTaskRepository struct {
	db db.Provider
}

// NewPgTaskRepository creates a db.Provider-backed task repository.
func NewPgTaskRepository(db db.Provider) *PgTaskRepository {
	return &PgTaskRepository{db: db}
}

func (r *PgTaskRepository) Create(ctx context.Context, task Task) error {
	payload, _ := json.Marshal(task.Payload)
	var err error
	if r.db.IsPostgres() {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO scheduled_tasks (id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, next_run_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)`,
			task.ID, task.OrganizationID, task.AgentID, task.Name,
			string(task.Schedule.Type), task.Schedule.At, task.Schedule.IntervalS, task.Schedule.Expression,
			string(task.Status), payload, task.CreatedAt, task.NextRunAt,
		)
	} else {
		_, err = r.db.ExecContext(ctx, `
			INSERT INTO scheduled_tasks (id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, next_run_at)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
			task.ID, task.OrganizationID, task.AgentID, task.Name,
			string(task.Schedule.Type), task.Schedule.At, task.Schedule.IntervalS, task.Schedule.Expression,
			string(task.Status), string(payload), task.CreatedAt, task.NextRunAt,
		)
	}
	if err != nil {
		return fmt.Errorf("db: create task: %w", err)
	}
	return nil
}

func (r *PgTaskRepository) Get(ctx context.Context, id string) (Task, error) {
	task := Task{}
	var schedType, status string
	var payload []byte
	var err error

	if r.db.IsPostgres() {
		err = r.db.QueryRowContext(ctx, `
			SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
			FROM scheduled_tasks WHERE id = $1`, id).Scan(
			&task.ID, &task.OrganizationID, &task.AgentID, &task.Name,
			&schedType, &task.Schedule.At, &task.Schedule.IntervalS, &task.Schedule.Expression,
			&status, &payload, &task.CreatedAt, &task.LastRunAt, &task.NextRunAt,
		)
	} else {
		err = r.db.QueryRowContext(ctx, `
			SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
			FROM scheduled_tasks WHERE id = ?`, id).Scan(
			&task.ID, &task.OrganizationID, &task.AgentID, &task.Name,
			&schedType, &task.Schedule.At, &task.Schedule.IntervalS, &task.Schedule.Expression,
			&status, &payload, &task.CreatedAt, &task.LastRunAt, &task.NextRunAt,
		)
	}
	if err != nil {
		return Task{}, fmt.Errorf("db: get task: %w", err)
	}
	task.Schedule.Type = ScheduleType(schedType)
	task.Status = TaskStatus(status)
	task.Payload = json.RawMessage(payload)
	return task, nil
}

func (r *PgTaskRepository) ListForOrg(ctx context.Context, orgID string) ([]Task, error) {
	var rows interface {
		Next() bool
		Scan(...any) error
		Close() error
	}
	var err error

	if r.db.IsPostgres() {
		rows, err = r.db.QueryContext(ctx, `
			SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
			FROM scheduled_tasks WHERE organization_id = $1 ORDER BY created_at`, orgID)
	} else {
		rows, err = r.db.QueryContext(ctx, `
			SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
			FROM scheduled_tasks WHERE organization_id = ? ORDER BY created_at`, orgID)
	}
	if err != nil {
		return nil, fmt.Errorf("db: list tasks: %w", err)
	}
	defer rows.Close()

	var tasks []Task
	for rows.Next() {
		var t Task
		var schedType, status string
		var payload []byte
		if err := rows.Scan(
			&t.ID, &t.OrganizationID, &t.AgentID, &t.Name,
			&schedType, &t.Schedule.At, &t.Schedule.IntervalS, &t.Schedule.Expression,
			&status, &payload, &t.CreatedAt, &t.LastRunAt, &t.NextRunAt,
		); err != nil {
			return nil, fmt.Errorf("db: scan task: %w", err)
		}
		t.Schedule.Type = ScheduleType(schedType)
		t.Status = TaskStatus(status)
		t.Payload = json.RawMessage(payload)
		tasks = append(tasks, t)
	}
	return tasks, nil
}

// PollDue claims due tasks. SQLite does not support FOR UPDATE SKIP LOCKED
func (r *PgTaskRepository) PollDue(ctx context.Context) ([]Task, error) {
	tx, err := r.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("db: begin poll tx: %w", err)
	}
	defer func() { _ = tx.Rollback() }()

	var rows interface {
		Next() bool
		Scan(...any) error
		Close() error
	}

	if r.db.IsPostgres() {
		rows, err = tx.QueryContext(ctx, `
			SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
			FROM scheduled_tasks
			WHERE status = 'pending' AND next_run_at <= NOW()
			FOR UPDATE SKIP LOCKED`)
	} else {
		// SQLite doesn't support FOR UPDATE SKIP LOCKED
		rows, err = tx.QueryContext(ctx, `
			SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
			FROM scheduled_tasks
			WHERE status = 'pending' AND next_run_at <= CURRENT_TIMESTAMP`)
	}

	if err != nil {
		return nil, fmt.Errorf("db: poll due: %w", err)
	}
	defer rows.Close()

	var tasks []Task
	now := time.Now().UTC()
	for rows.Next() {
		var t Task
		var schedType, status string
		var payload []byte
		if err := rows.Scan(
			&t.ID, &t.OrganizationID, &t.AgentID, &t.Name,
			&schedType, &t.Schedule.At, &t.Schedule.IntervalS, &t.Schedule.Expression,
			&status, &payload, &t.CreatedAt, &t.LastRunAt, &t.NextRunAt,
		); err != nil {
			return nil, fmt.Errorf("db: scan due task: %w", err)
		}
		t.Schedule.Type = ScheduleType(schedType)
		t.Status = TaskStatus(status)
		t.Payload = json.RawMessage(payload)
		tasks = append(tasks, t)
	}
	rows.Close()

	for _, t := range tasks {
		// Mark as running within this transaction.
		if r.db.IsPostgres() {
			if _, err := tx.ExecContext(ctx, "UPDATE scheduled_tasks SET status='running', last_run_at=$2 WHERE id=$1", t.ID, now); err != nil {
				return nil, fmt.Errorf("db: mark running pg: %w", err)
			}
		} else {
			if _, err := tx.ExecContext(ctx, "UPDATE scheduled_tasks SET status='running', last_run_at=? WHERE id=?", now, t.ID); err != nil {
				return nil, fmt.Errorf("db: mark running sqlite: %w", err)
			}
		}
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("db: commit poll: %w", err)
	}
	return tasks, nil
}

func (r *PgTaskRepository) UpdateStatus(ctx context.Context, id string, status TaskStatus, reschedule bool) error {
	if reschedule {
		if r.db.IsPostgres() {
			_, err := r.db.ExecContext(ctx, `
				UPDATE scheduled_tasks
				SET status = 'pending', next_run_at = NOW() + (interval_s * INTERVAL '1 second')
				WHERE id = $1`, id)
			return err
		} else {
			// SQLite
			_, err := r.db.ExecContext(ctx, `
				UPDATE scheduled_tasks
				SET status = 'pending', next_run_at = datetime(CURRENT_TIMESTAMP, '+' || interval_s || ' seconds')
				WHERE id = ?`, id)
			return err
		}
	}

	if r.db.IsPostgres() {
		_, err := r.db.ExecContext(ctx, "UPDATE scheduled_tasks SET status = $2 WHERE id = $1", id, string(status))
		return err
	}
	_, err := r.db.ExecContext(ctx, "UPDATE scheduled_tasks SET status = ? WHERE id = ?", string(status), id)
	return err
}

func (r *PgTaskRepository) Cancel(ctx context.Context, id string) error {
	if r.db.IsPostgres() {
		_, err := r.db.ExecContext(ctx, "UPDATE scheduled_tasks SET status = 'cancelled' WHERE id = $1", id)
		return err
	}
	_, err := r.db.ExecContext(ctx, "UPDATE scheduled_tasks SET status = 'cancelled' WHERE id = ?", id)
	return err
}
