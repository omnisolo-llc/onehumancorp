package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SharedTask represents a shared task distributed across agents.
type SharedTask struct {
	ID              string
	MissionID       string
	Title           string
	Description     string
	AssignedAgentID string
	Status          string // PENDING, IN_PROGRESS, COMPLETED, FAILED
	Priority        string
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

// TaskManager manages the shared tasks list
type TaskManager struct {
	db db.Provider
}

// NewTaskManager creates a new TaskManager.
func NewTaskManager(provider db.Provider) *TaskManager {
	return &TaskManager{db: provider}
}

// CreateTask creates a new shared task.
func (tm *TaskManager) CreateTask(ctx context.Context, missionID, title, description, priority string) (*SharedTask, error) {
	if priority == "" {
		priority = "P2"
	}

	var task SharedTask
	var query string
	var err error

	if tm.db.IsSQLite() {
		// SQLite might not support RETURNING for all columns gracefully depending on version,
		// but since we override UUID PRIMARY KEY DEFAULT gen_random_uuid() to TEXT, we need to generate UUID or rely on it.
		// Wait, if we rely on TEXT, we should generate an ID in go
		id := generateID() // Helper func
		query = `
			INSERT INTO shared_tasks (id, mission_id, title, description, priority, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			RETURNING id, mission_id, title, description, priority, status, created_at, updated_at
		`
		err = tm.db.QueryRow(ctx, query, id, missionID, title, description, priority).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		query = `
			INSERT INTO shared_tasks (mission_id, title, description, priority)
			VALUES ($1, $2, $3, $4)
			RETURNING id, mission_id, title, description, priority, status, created_at, updated_at
		`
		err = tm.db.QueryRow(ctx, query, missionID, title, description, priority).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to create task: %w", err)
	}

	return &task, nil
}

// ClaimTask attempts to claim a PENDING task for the given agentID.
// It uses row-level locking (FOR UPDATE) in Postgres, and relies on SQLite's lock mechanism
// to prevent race conditions.
func (tm *TaskManager) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	var errQuery error

	if tm.db.IsSQLite() {
		// SQLite doesn't support FOR UPDATE, but `Begin` handles concurrent writes lock.
		query := `
			SELECT id, mission_id, title, description, priority, status, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		`
		errQuery = tx.QueryRow(ctx, query).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		// PostgreSQL with SKIP LOCKED
		query := `
			SELECT id, mission_id, title, description, priority, status, created_at, updated_at
			FROM shared_tasks
			WHERE status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
		errQuery = tx.QueryRow(ctx, query).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to find pending task: %w", errQuery)
	}

	// Update task status to IN_PROGRESS
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'PENDING'
	`
	rowsAffected, err := tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if rowsAffected == 0 {
		// Task was likely claimed by another worker concurrently.
		return nil, nil
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID
	return &task, nil
}

// CompleteTask marks a task as completed.
func (tm *TaskManager) CompleteTask(ctx context.Context, taskID, agentID string) error {
	query := `
		UPDATE shared_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS'
	`
	res, err := tm.db.Exec(ctx, query, taskID, agentID)
	if err != nil {
		return fmt.Errorf("failed to complete task: %w", err)
	}

	if res == 0 {
		return errors.New("task not found or not assigned to agent")
	}

	return nil
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	return fmt.Sprintf("%d", time.Now().UnixNano())
}
