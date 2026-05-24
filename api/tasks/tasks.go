package tasks

import (
	"database/sql"
	"fmt"
	"strconv"
	"strings"
	"time"
)

type SharedTask struct {
	ID              string
	TenantID        string
	EpicID          sql.NullString
	Title           string
	Description     sql.NullString
	Priority        string
	Status          string
	AssignedAgentID sql.NullString
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

type TaskDAO interface {
	CreateTask(task *SharedTask) error
	GetTask(tenantID, id string) (*SharedTask, error)
	UpdateTask(task *SharedTask) error
	DeleteTask(tenantID, id string) error
	ListTasks(tenantID, cursor string, limit int) ([]*SharedTask, string, error)
}

type taskDAOImpl struct {
	db         *sql.DB
	isPostgres bool
}

func NewTaskDAO(db *sql.DB, isPostgres bool) TaskDAO {
	return &taskDAOImpl{db: db, isPostgres: isPostgres}
}

// rebind replaces ? with $1, $2, etc. if we're using postgres
func (d *taskDAOImpl) rebind(query string) string {
	if !d.isPostgres {
		return query
	}
	parts := strings.Split(query, "?")
	if len(parts) == 1 {
		return query
	}
	var b strings.Builder
	for i := 0; i < len(parts)-1; i++ {
		b.WriteString(parts[i])
		b.WriteString("$")
		b.WriteString(strconv.Itoa(i + 1))
	}
	b.WriteString(parts[len(parts)-1])
	return b.String()
}

func (d *taskDAOImpl) CreateTask(task *SharedTask) error {
	query := `INSERT INTO shared_task_list (id, tenant_id, epic_id, title, description, priority, status, assigned_agent_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`
	query = d.rebind(query)

	now := time.Now()
	task.CreatedAt = now
	task.UpdatedAt = now

	_, err := d.db.Exec(query, task.ID, task.TenantID, task.EpicID, task.Title, task.Description, task.Priority, task.Status, task.AssignedAgentID, task.CreatedAt, task.UpdatedAt)
	return err
}

func (d *taskDAOImpl) GetTask(tenantID, id string) (*SharedTask, error) {
	query := `SELECT id, tenant_id, epic_id, title, description, priority, status, assigned_agent_id, created_at, updated_at FROM shared_task_list WHERE id = ? AND tenant_id = ?`
	query = d.rebind(query)

	row := d.db.QueryRow(query, id, tenantID)

	var task SharedTask
	err := row.Scan(&task.ID, &task.TenantID, &task.EpicID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.AssignedAgentID, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, fmt.Errorf("task not found")
		}
		return nil, err
	}

	return &task, nil
}

func (d *taskDAOImpl) UpdateTask(task *SharedTask) error {
	query := `UPDATE shared_task_list SET epic_id = ?, title = ?, description = ?, priority = ?, status = ?, assigned_agent_id = ?, updated_at = ? WHERE id = ? AND tenant_id = ?`
	query = d.rebind(query)

	task.UpdatedAt = time.Now()

	_, err := d.db.Exec(query, task.EpicID, task.Title, task.Description, task.Priority, task.Status, task.AssignedAgentID, task.UpdatedAt, task.ID, task.TenantID)
	return err
}

func (d *taskDAOImpl) DeleteTask(tenantID, id string) error {
	query := `DELETE FROM shared_task_list WHERE id = ? AND tenant_id = ?`
	query = d.rebind(query)

	_, err := d.db.Exec(query, id, tenantID)
	return err
}

func (d *taskDAOImpl) ListTasks(tenantID, cursor string, limit int) ([]*SharedTask, string, error) {
	if limit <= 0 || limit > 20 {
		limit = 20
	}

	query := `SELECT id, tenant_id, epic_id, title, description, priority, status, assigned_agent_id, created_at, updated_at FROM shared_task_list WHERE tenant_id = ?`
	var args []interface{}
	args = append(args, tenantID)

	if cursor != "" {
		query += ` AND id > ?`
		args = append(args, cursor)
	}

	query += ` ORDER BY id ASC LIMIT ?`
	args = append(args, limit)

	query = d.rebind(query)

	rows, err := d.db.Query(query, args...)
	if err != nil {
		return nil, "", err
	}
	defer rows.Close()

	var tasks []*SharedTask
	for rows.Next() {
		var task SharedTask
		if err := rows.Scan(&task.ID, &task.TenantID, &task.EpicID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.AssignedAgentID, &task.CreatedAt, &task.UpdatedAt); err != nil {
			return nil, "", err
		}
		tasks = append(tasks, &task)
	}

	if err := rows.Err(); err != nil {
		return nil, "", err
	}

	var nextCursor string
	if len(tasks) == limit {
		nextCursor = tasks[len(tasks)-1].ID
	}

	return tasks, nextCursor, nil
}
