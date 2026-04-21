package tasks

import (
	"context"
	"database/sql"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/onehumancorp/mono/srcs/server/db"
)

var TasksCompleted = prometheus.NewCounterVec(
	prometheus.CounterOpts{
		Name: "ohc.tasks.completed",
		Help: "Number of tasks completed",
	},
	[]string{"status"},
)

func init() {
	prometheus.MustRegister(TasksCompleted)
}

type TaskQueue struct {
	provider db.Provider
}

func NewTaskQueue(provider db.Provider) *TaskQueue {
	return &TaskQueue{provider: provider}
}

type Task struct {
	ID        string
	Title     string
	Status    string
	Assignee  *string
}

func (q *TaskQueue) CreateTask(ctx context.Context, id, title string) error {
	query := `INSERT INTO shared_tasks (id, title) VALUES ($1, $2)`
	_, err := q.provider.Exec(ctx, query, id, title)
	return err
}

func (q *TaskQueue) ListTasks(ctx context.Context) ([]Task, error) {
	query := `SELECT id, title, status, assignee FROM shared_tasks ORDER BY created_at DESC`
	rows, err := q.provider.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []Task
	for rows.Next() {
		var t Task
		if err := rows.Scan(&t.ID, &t.Title, &t.Status, &t.Assignee); err != nil {
			return nil, err
		}
		tasks = append(tasks, t)
	}
	return tasks, nil
}

func (q *TaskQueue) ClaimTask(ctx context.Context, agentID string) (string, error) {
	query := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assignee = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM shared_tasks
			WHERE status = 'PENDING'
			ORDER BY created_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id
	`
	row := q.provider.QueryRow(ctx, query, agentID)
	var id string
	err := row.Scan(&id)
	if err != nil {
		if err == sql.ErrNoRows {
			return "", nil
		}
		return "", err
	}
	return id, nil
}

func (q *TaskQueue) CompleteTask(ctx context.Context, taskID string) error {
	query := `UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1`
	_, err := q.provider.Exec(ctx, query, taskID)
	if err == nil {
		TasksCompleted.WithLabelValues("COMPLETED").Inc()
	}
	return err
}
