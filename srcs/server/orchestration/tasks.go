package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"log/slog"
	"os"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// SharedTask represents a mission or subtask that agents can claim.
type SharedTask struct {
	ID              string
	MissionID       string
	Title           string
	Description     string
	AssignedAgentID string
	Status          string // PENDING, IN_PROGRESS, COMPLETED, FAILED
	Priority        string // P0, P1, P2
}

// SharedTaskList manages distributed locking and assignment of tasks.
type SharedTaskList struct {
	dbProvider db.Provider
	useRedis   bool
}

// NewSharedTaskList creates a new task list manager.
func NewSharedTaskList(provider db.Provider) *SharedTaskList {
	return &SharedTaskList{
		dbProvider: provider,
		useRedis:   os.Getenv("OHC_MULTITENANT") == "true",
	}
}

// ClaimTask attempts to claim a PENDING task and transitions it to IN_PROGRESS.
// This requires a locking mechanism to prevent race conditions.
func (s *SharedTaskList) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	// Start a transaction
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	var row db.Row

	if s.useRedis {
		// IN MULTITENANT cloud mode, we would ideally use Redis Distributed Locks.
		// For simplicity, we just use the row lock here as well, since Postgres supports FOR UPDATE SKIP LOCKED
		row = tx.QueryRow(ctx, `
			SELECT id, mission_id, title, description, assigned_agent_id, status, priority
			FROM shared_tasks
			WHERE status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		`)
	} else {
		// IN STANDALONE mode, SQLite handles locking during updates, but doesn't support FOR UPDATE.
		row = tx.QueryRow(ctx, `
			SELECT id, mission_id, title, description, assigned_agent_id, status, priority
			FROM shared_tasks
			WHERE status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		`)
	}

    var desc sql.NullString
    var assigned sql.NullString

	err = row.Scan(&task.ID, &task.MissionID, &task.Title, &desc, &assigned, &task.Status, &task.Priority)
	if err != nil {
		// Usually standard DB error indicates no rows by sql.ErrNoRows.
		// For our provider, if scan fails due to no rows, we can check string representation.
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil // No pending tasks
		}
		// Try to handle gracefully if not specifically ErrNoRows, in tests it could be different
		// but let's assume standard behavior. We will just return nil, nil if there is no match
		// but let's be careful not to swallow real errors.
		return nil, fmt.Errorf("scan pending task: %w", err)
	}

    if desc.Valid {
        task.Description = desc.String
    }
    if assigned.Valid {
        task.AssignedAgentID = assigned.String
    }

	// Update task to IN_PROGRESS
	task.AssignedAgentID = agentID
	task.Status = "IN_PROGRESS"

	_, err = tx.Exec(ctx, `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'PENDING'
	`, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("update task: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("commit task claim: %w", err)
	}

	slog.Debug("Agent claimed task", "agent_id", agentID, "task_id", task.ID)
	return &task, nil
}

// AddTask creates a new task in the queue.
func (s *SharedTaskList) AddTask(ctx context.Context, missionID, title, description, priority string) (*SharedTask, error) {
	taskID := uuid.New().String()
	_, err := s.dbProvider.Exec(ctx, `
		INSERT INTO shared_tasks (id, mission_id, title, description, status, priority)
		VALUES ($1, $2, $3, $4, 'PENDING', $5)
	`, taskID, missionID, title, description, priority)
	if err != nil {
		return nil, fmt.Errorf("insert task: %w", err)
	}

	return &SharedTask{
		ID:          taskID,
		MissionID:   missionID,
		Title:       title,
		Description: description,
		Status:      "PENDING",
		Priority:    priority,
	}, nil
}

// CompleteTask marks a task as COMPLETED.
func (s *SharedTaskList) CompleteTask(ctx context.Context, taskID string) error {
	_, err := s.dbProvider.Exec(ctx, `
		UPDATE shared_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`, taskID)
	return err
}
