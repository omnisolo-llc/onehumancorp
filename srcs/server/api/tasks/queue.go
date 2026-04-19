package tasks

import (
	"context"
	"database/sql"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/onehumancorp/mono/srcs/server/db"
)

var TasksCompleted = prometheus.NewCounterVec(
	prometheus.CounterOpts{
		Name: "tasks_completed_total",
		Help: "Total number of tasks completed",
	},
	[]string{"organization_id", "status"},
)

func init() {
	prometheus.MustRegister(TasksCompleted)
}

type Task struct {
	ID             string    `json:"id"`
	OrganizationID string    `json:"organization_id"`
	Title          string    `json:"title"`
	Description    string    `json:"description"`
	Status         string    `json:"status"`
	Assignee       string    `json:"assignee"`
	CreatedAt      time.Time `json:"created_at"`
	UpdatedAt      time.Time `json:"updated_at"`
}

type Queue struct {
	provider db.Provider
}

func NewQueue(provider db.Provider) *Queue {
	return &Queue{provider: provider}
}

func (q *Queue) ClaimTask(ctx context.Context, organizationID, agentID string) (*Task, error) {
	var t Task
	query := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assignee = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM shared_tasks
			WHERE status = 'PENDING' AND organization_id = $2
			ORDER BY created_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, organization_id, title, description, status, assignee, created_at, updated_at
	`

	// db.Provider strips "FOR UPDATE SKIP LOCKED" automatically if it's SQLite.
	err := q.provider.QueryRow(ctx, query, agentID, organizationID).Scan(
		&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Status, &t.Assignee, &t.CreatedAt, &t.UpdatedAt,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, err
	}
	return &t, nil
}

func (q *Queue) AddTask(ctx context.Context, t *Task) error {
	query := `
		INSERT INTO shared_tasks (organization_id, title, description)
		VALUES ($1, $2, $3)
		RETURNING id, status, created_at, updated_at
	`
	return q.provider.QueryRow(ctx, query, t.OrganizationID, t.Title, t.Description).Scan(
		&t.ID, &t.Status, &t.CreatedAt, &t.UpdatedAt,
	)
}
