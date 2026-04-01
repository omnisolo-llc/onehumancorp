package scheduler

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// PgTaskRepository implements TaskRepository backed by PostgreSQL.
// It uses SELECT ... FOR UPDATE SKIP LOCKED to ensure that concurrent
// replicas never execute the same task twice.
type PgTaskRepository struct {
	pool db.Provider
}

// NewPgTaskRepository creates a Postgres-backed task repository.
func NewPgTaskRepository(pool db.Provider) *PgTaskRepository {
	return &PgTaskRepository{pool: pool}
}

func (r *PgTaskRepository) Create(ctx context.Context, task Task) error {
	payload, _ := json.Marshal(task.Payload)
	_, err := r.pool.Exec(ctx, `
		INSERT INTO scheduled_tasks (id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, next_run_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)`,
		task.ID, task.OrganizationID, task.AgentID, task.Name,
		string(task.Schedule.Type), task.Schedule.At, task.Schedule.IntervalS, task.Schedule.Expression,
		string(task.Status), string(payload), task.CreatedAt, task.NextRunAt,
	)
	if err != nil {
		return fmt.Errorf("pg: create task: %w", err)
	}
	return nil
}

func (r *PgTaskRepository) Get(ctx context.Context, id string) (Task, error) {
	task := Task{}
	var schedType, status string
	var payload string
	err := r.pool.QueryRow(ctx, `
		SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
		FROM scheduled_tasks WHERE id = $1`, id).Scan(
		&task.ID, &task.OrganizationID, &task.AgentID, &task.Name,
		&schedType, &task.Schedule.At, &task.Schedule.IntervalS, &task.Schedule.Expression,
		&status, &payload, &task.CreatedAt, &task.LastRunAt, &task.NextRunAt,
	)
	if err != nil {
		return Task{}, fmt.Errorf("pg: get task: %w", err)
	}
	task.Schedule.Type = ScheduleType(schedType)
	task.Status = TaskStatus(status)
	task.Payload = json.RawMessage(payload)
	return task, nil
}

func (r *PgTaskRepository) ListForOrg(ctx context.Context, orgID string) ([]Task, error) {
	rows, err := r.pool.Query(ctx, `
		SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
		FROM scheduled_tasks WHERE organization_id = $1 ORDER BY created_at`, orgID)
	if err != nil {
		return nil, fmt.Errorf("pg: list tasks: %w", err)
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
			return nil, fmt.Errorf("pg: scan task: %w", err)
		}
		t.Schedule.Type = ScheduleType(schedType)
		t.Status = TaskStatus(status)
		t.Payload = json.RawMessage(payload)
		tasks = append(tasks, t)
	}
	return tasks, nil
}

// PollDue uses SELECT ... FOR UPDATE SKIP LOCKED to claim due tasks
// atomically.  This prevents duplicate execution across replicas.
func (r *PgTaskRepository) PollDue(ctx context.Context) ([]Task, error) {
	tx, err := r.pool.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("pg: begin poll tx: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	query := `
		SELECT id, organization_id, agent_id, name, schedule_type, schedule_at, interval_s, expression, status, payload, created_at, last_run_at, next_run_at
		FROM scheduled_tasks
		WHERE status = 'pending' AND next_run_at <= CURRENT_TIMESTAMP`

	if !r.pool.IsSQLite() {
		query += " FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("pg: poll due: %w", err)
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
			return nil, fmt.Errorf("pg: scan due task: %w", err)
		}
		t.Schedule.Type = ScheduleType(schedType)
		t.Status = TaskStatus(status)
		t.Payload = json.RawMessage(payload)
		tasks = append(tasks, t)
	}
	rows.Close()

	for _, t := range tasks {
		// Mark as running within this transaction.
		if _, err := tx.Exec(ctx, "UPDATE scheduled_tasks SET status='running', last_run_at=$2 WHERE id=$1", t.ID, now); err != nil {
			return nil, fmt.Errorf("pg: mark running: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("pg: commit poll: %w", err)
	}
	return tasks, nil
}

func (r *PgTaskRepository) UpdateStatus(ctx context.Context, id string, status TaskStatus, reschedule bool) error {
	if reschedule {
		if r.pool.IsSQLite() {
			_, err := r.pool.Exec(ctx, `
				UPDATE scheduled_tasks
				SET status = 'pending', next_run_at = datetime(CURRENT_TIMESTAMP, '+' || interval_s || ' seconds')
				WHERE id = $1`, id)
			return err
		} else {
			_, err := r.pool.Exec(ctx, `
				UPDATE scheduled_tasks
				SET status = 'pending', next_run_at = CURRENT_TIMESTAMP + (interval_s * INTERVAL '1 second')
				WHERE id = $1`, id)
			return err
		}
	}
	_, err := r.pool.Exec(ctx, "UPDATE scheduled_tasks SET status = $2 WHERE id = $1", id, string(status))
	return err
}

func (r *PgTaskRepository) Cancel(ctx context.Context, id string) error {
	_, err := r.pool.Exec(ctx, "UPDATE scheduled_tasks SET status = 'cancelled' WHERE id = $1", id)
	return err
}
