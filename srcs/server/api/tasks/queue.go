package tasks

import (
	"context"
	"database/sql"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/prometheus/client_golang/prometheus"
)

var (
	tasksCompleted = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_tasks_completed_total",
			Help: "Total number of tasks completed",
		},
		[]string{"status"},
	)
)

func init() {
	prometheus.MustRegister(tasksCompleted)
}

type TaskQueue struct {
	dbProvider db.Provider
}

func NewTaskQueue(dbProvider db.Provider) *TaskQueue {
	return &TaskQueue{dbProvider: dbProvider}
}

type Task struct {
	ID           string   `json:"id"`
	Title        string   `json:"title"`
	Status       string   `json:"status"`
	Assignee     string   `json:"assignee"`
	Dependencies []string `json:"dependencies"`
	CreatedAt    time.Time`json:"created_at"`
	UpdatedAt    time.Time`json:"updated_at"`
}

func (q *TaskQueue) ListTasks(ctx context.Context) ([]Task, error) {
	rows, err := q.dbProvider.Query(ctx, "SELECT id, title, status, assignee, created_at, updated_at FROM shared_tasks ORDER BY created_at DESC")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []Task
	for rows.Next() {
		var t Task
		var assignee sql.NullString
		var createdAt, updatedAt db.FlexTime
		if err := rows.Scan(&t.ID, &t.Title, &t.Status, &assignee, &createdAt, &updatedAt); err != nil {
			return nil, err
		}
		if assignee.Valid {
			t.Assignee = assignee.String
		}
		t.CreatedAt = createdAt.Time
		t.UpdatedAt = updatedAt.Time
		tasks = append(tasks, t)
	}
	return tasks, nil
}

func (q *TaskQueue) ClaimTask(ctx context.Context, agentID string) (*Task, error) {
	tx, err := q.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var query string
	if q.dbProvider.IsSQLite() {
		query = `
			UPDATE shared_tasks
			SET status = 'ASSIGNED', assignee = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM shared_tasks
				WHERE status = 'PENDING'
				ORDER BY created_at ASC
				LIMIT 1
			)
			RETURNING id, title, status, assignee, created_at, updated_at
		`
	} else {
		query = `
			UPDATE shared_tasks
			SET status = 'ASSIGNED', assignee = $1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM shared_tasks
				WHERE status = 'PENDING'
				ORDER BY created_at ASC
				LIMIT 1
				FOR UPDATE SKIP LOCKED
			)
			RETURNING id, title, status, assignee, created_at, updated_at
		`
	}

	var t Task
	var createdAt, updatedAt db.FlexTime
	var assignee sql.NullString
	err = tx.QueryRow(ctx, query, agentID).Scan(&t.ID, &t.Title, &t.Status, &assignee, &createdAt, &updatedAt)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No tasks available
		}
		return nil, err
	}
	if assignee.Valid {
		t.Assignee = assignee.String
	}
	t.CreatedAt = createdAt.Time
	t.UpdatedAt = updatedAt.Time

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return &t, nil
}

func (q *TaskQueue) CompleteTask(ctx context.Context, taskID string) error {
	_, err := q.dbProvider.Exec(ctx, "UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1", taskID)
	if err == nil {
		tasksCompleted.WithLabelValues("COMPLETED").Inc()
	}
	return err
}
