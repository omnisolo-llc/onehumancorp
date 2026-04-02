package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

var (
	ErrTaskNotFound = errors.New("task not found")
	ErrTaskLocked   = errors.New("task locked by another agent")
)

type SharedTask struct {
	ID              string    `json:"id"`
	MissionID       string    `json:"mission_id"`
	Title           string    `json:"title"`
	Description     string    `json:"description"`
	AssignedAgentID string    `json:"assigned_agent_id"`
	Status          string    `json:"status"`
	Priority        string    `json:"priority"`
	CreatedAt       time.Time `json:"created_at"`
	UpdatedAt       time.Time `json:"updated_at"`
}

type TaskManager struct {
	db db.Provider
}

func NewTaskManager(provider db.Provider) *TaskManager {
	return &TaskManager{db: provider}
}

// ClaimTask attempts to claim a task.
// If in cloud mode, it ideally uses a Redis lock (simulated with row-level locking here since Redis is not easily accessible from standard DB connections, or we can use Postgres row level locking).
// For simplicity and hybrid compatibility, we use DB transactions.
func (m *TaskManager) ClaimTask(ctx context.Context, taskID, agentID string) error {
	tx, err := m.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback()

	// Use row-level locking if possible, but SQLite doesn't support SELECT ... FOR UPDATE.
	// Since we handle both, we just check status and update in one atomic go.

	// Wait, SQLite has FOR UPDATE? No. So we do an UPDATE ... WHERE status = 'PENDING' AND id = ?
	// This works identically well for both PG and SQLite!

	res, err := tx.Exec(ctx,
		"UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND status = 'PENDING'",
		agentID, taskID)
	if err != nil {
		// modernc.sqlite issue fallback
		res, err = tx.Exec(ctx,
			"UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND status = 'PENDING'",
			agentID, taskID)
		if err != nil {
			return fmt.Errorf("update task: %w", err)
		}
	}

	if res == 0 {
		// Task either doesn't exist, isn't pending, or is already claimed.
		// Let's check if it exists at all to give a precise error.
		var status string
		var count int
		row := tx.QueryRow(ctx, "SELECT COUNT(*), COALESCE(status, '') FROM shared_tasks WHERE id = $1 GROUP BY status", taskID)
		err := row.Scan(&count, &status)
		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				row = tx.QueryRow(ctx, "SELECT COUNT(*), COALESCE(status, '') FROM shared_tasks WHERE id = ?", taskID)
				err = row.Scan(&count, &status)
			}
		}
		if err != nil && !errors.Is(err, sql.ErrNoRows) {
			// ignore error
		}

		if count == 0 {
			return ErrTaskNotFound
		}
		return ErrTaskLocked
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("commit tx: %w", err)
	}

	return nil
}

func (m *TaskManager) UpdateTaskStatus(ctx context.Context, taskID, agentID, status string) error {
	res, err := m.db.Exec(ctx,
		"UPDATE shared_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND assigned_agent_id = $3",
		status, taskID, agentID)
	if err != nil {
		res, err = m.db.Exec(ctx,
			"UPDATE shared_tasks SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND assigned_agent_id = ?",
			status, taskID, agentID)
		if err != nil {
			return fmt.Errorf("update task status: %w", err)
		}
	}

	if res == 0 {
		return errors.New("could not update task (not found, unauthorized, or invalid state)")
	}
	return nil
}

func (m *TaskManager) CreateTask(ctx context.Context, task SharedTask) (string, error) {
	// Let DB generate UUID if empty (except sqlite fallback)
	if task.ID == "" && m.db.IsSQLite() {
		task.ID = fmt.Sprintf("task-%d", time.Now().UnixNano())
	}

	var newID string
	if m.db.IsSQLite() {
		// SQLite fallback
		_, err := m.db.Exec(ctx,
			"INSERT INTO shared_tasks (id, mission_id, title, description, priority) VALUES (?, ?, ?, ?, ?)",
			task.ID, task.MissionID, task.Title, task.Description, task.Priority)
		if err != nil {
			return "", err
		}
		newID = task.ID
	} else {
		// Postgres
		row := m.db.QueryRow(ctx,
			"INSERT INTO shared_tasks (mission_id, title, description, priority) VALUES ($1, $2, $3, $4) RETURNING id",
			task.MissionID, task.Title, task.Description, task.Priority)
		if err := row.Scan(&newID); err != nil {
			return "", err
		}
	}

	return newID, nil
}

func (m *TaskManager) ListPendingTasks(ctx context.Context) ([]SharedTask, error) {
	rows, err := m.db.Query(ctx, "SELECT id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at FROM shared_tasks WHERE status = 'PENDING' ORDER BY priority ASC, created_at ASC")
	if err != nil {
		return nil, fmt.Errorf("query tasks: %w", err)
	}
	defer rows.Close()

	var tasks []SharedTask
	for rows.Next() {
		var t SharedTask
		var assigned sql.NullString
		var desc sql.NullString
		if err := rows.Scan(&t.ID, &t.MissionID, &t.Title, &desc, &assigned, &t.Status, &t.Priority, &t.CreatedAt, &t.UpdatedAt); err != nil {
			// fallback for sqlite which might return strings instead of time.Time
			var cat, uat string
			// We can't really do this gracefully inline if the Scan failed because rows pointer advanced,
			// but since modernc.sqlite / pgx usually handle time.Time nicely, we directly scan into time.Time.
			return nil, err
		}
		t.AssignedAgentID = assigned.String
		t.Description = desc.String
		tasks = append(tasks, t)
	}
	return tasks, nil
}
