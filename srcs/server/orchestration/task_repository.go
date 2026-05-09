package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"sync"
	"time"

	"github.com/google/uuid"
)

type TaskRepository interface {
	CreateTask(ctx context.Context, task *Task) error
	GetTask(ctx context.Context, tenantID string, id string) (*Task, error)
	ListTasks(ctx context.Context, tenantID string) ([]*Task, error)
	UpdateTask(ctx context.Context, task *Task) error
	ClaimTask(ctx context.Context, tenantID string, id string, agentID string) (*Task, error)
}

type DBTaskRepository struct {
	db       *sql.DB
	// For SQLite, need a lock for reliable claiming in standalone mode
	mu       sync.Mutex
	isSQLite bool
}

func NewDBTaskRepository(db *sql.DB) *DBTaskRepository {
	var v string
	err := db.QueryRow("SELECT sqlite_version()").Scan(&v)
	isSQLite := err == nil
	return &DBTaskRepository{db: db, isSQLite: isSQLite}
}

func (r *DBTaskRepository) CreateTask(ctx context.Context, task *Task) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}

	query := `
		INSERT INTO ohc_tasks (
			id, tenant_id, title, description, status, assigned_agent_id,
			priority, payload, parent_task_id, workflow_state, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8, $9, $10, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
		)
	`
	var payloadBytes []byte
	if task.Payload != nil {
		payloadBytes = []byte(*task.Payload)
	}

	_, err := r.db.ExecContext(ctx, query,
		task.ID, task.TenantID, task.Title, task.Description, task.Status, task.AssignedAgentID,
		task.Priority, payloadBytes, task.ParentTaskID, task.WorkflowState,
	)
	return err
}

func (r *DBTaskRepository) GetTask(ctx context.Context, tenantID string, id string) (*Task, error) {
	query := `
		SELECT id, tenant_id, title, description, status, assigned_agent_id,
		priority, payload, parent_task_id, workflow_state, created_at, updated_at
		FROM ohc_tasks WHERE id = $1 AND tenant_id = $2
	`
	row := r.db.QueryRowContext(ctx, query, id, tenantID)

	task := &Task{}
	var payloadBytes []byte
	var createdAt, updatedAt interface{}

	err := row.Scan(
		&task.ID, &task.TenantID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID,
		&task.Priority, &payloadBytes, &task.ParentTaskID, &task.WorkflowState, &createdAt, &updatedAt,
	)

	if err != nil {
		if err == sql.ErrNoRows {
			return nil, errors.New("task not found")
		}
		return nil, err
	}

	task.CreatedAt = parseTime(createdAt)
	task.UpdatedAt = parseTime(updatedAt)

	if len(payloadBytes) > 0 {
		raw := json.RawMessage(payloadBytes)
		task.Payload = &raw
	}

	return task, nil
}

func (r *DBTaskRepository) ListTasks(ctx context.Context, tenantID string) ([]*Task, error) {
	query := `
		SELECT id, tenant_id, title, description, status, assigned_agent_id,
		priority, payload, parent_task_id, workflow_state, created_at, updated_at
		FROM ohc_tasks WHERE tenant_id = $1
	`
	rows, err := r.db.QueryContext(ctx, query, tenantID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var tasks []*Task
	for rows.Next() {
		task := &Task{}
		var payloadBytes []byte
		var createdAt, updatedAt interface{}

		err := rows.Scan(
			&task.ID, &task.TenantID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID,
			&task.Priority, &payloadBytes, &task.ParentTaskID, &task.WorkflowState, &createdAt, &updatedAt,
		)
		if err != nil {
			return nil, err
		}

		task.CreatedAt = parseTime(createdAt)
		task.UpdatedAt = parseTime(updatedAt)

		if len(payloadBytes) > 0 {
			raw := json.RawMessage(payloadBytes)
			task.Payload = &raw
		}
		tasks = append(tasks, task)
	}
	return tasks, nil
}

func (r *DBTaskRepository) UpdateTask(ctx context.Context, task *Task) error {
	query := `
		UPDATE ohc_tasks SET
			title = $1, description = $2, status = $3, assigned_agent_id = $4,
			priority = $5, payload = $6, parent_task_id = $7, workflow_state = $8,
			updated_at = CURRENT_TIMESTAMP
		WHERE id = $9 AND tenant_id = $10
	`
	var payloadBytes []byte
	if task.Payload != nil {
		payloadBytes = []byte(*task.Payload)
	}

	_, err := r.db.ExecContext(ctx, query,
		task.Title, task.Description, task.Status, task.AssignedAgentID,
		task.Priority, payloadBytes, task.ParentTaskID, task.WorkflowState, task.ID, task.TenantID,
	)
	return err
}

func (r *DBTaskRepository) ClaimTask(ctx context.Context, tenantID string, id string, agentID string) (*Task, error) {
	if r.isSQLite {
		r.mu.Lock()
		defer r.mu.Unlock()
	}

	tx, err := r.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	// Distributed lock via atomic DB update
	// To support standard SQL (both SQLite and Postgres) without RETURNING
	// we update and check rows affected.
	updateQuery := `
		UPDATE ohc_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND tenant_id = $3 AND status = 'PENDING'
	`
	res, err := tx.ExecContext(ctx, updateQuery, agentID, id, tenantID)
	if err != nil {
		return nil, err
	}

	rowsAffected, err := res.RowsAffected()
	if err != nil {
		return nil, err
	}

	if rowsAffected == 0 {
		return nil, errors.New("failed to claim task: task not found or not in PENDING state")
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	// Fetch and return the updated task
	return r.GetTask(ctx, tenantID, id)
}

func parseTime(t interface{}) time.Time {
	switch v := t.(type) {
	case time.Time:
		return v
	case string:
		parsed, _ := time.Parse(time.RFC3339, v)
		return parsed
	default:
		return time.Time{}
	}
}