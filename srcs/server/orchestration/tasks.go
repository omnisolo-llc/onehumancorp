package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"log/slog"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SharedTask struct {
	ID              string
	MissionID       string
	Title           string
	Description     string
	AssignedAgentID string
	Status          string
	Priority        string
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

type TaskManager struct {
	db *db.DB
}

func NewTaskManager(db *db.DB) *TaskManager {
	return &TaskManager{db: db}
}

func (tm *TaskManager) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	// Attempt to claim a pending task
	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	var assignedAgentID sql.NullString
	var description sql.NullString

	query := `
		SELECT id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING'
		ORDER BY priority ASC, created_at ASC
		LIMIT 1
	`

	// PostgreSQL uses FOR UPDATE SKIP LOCKED. SQLite doesn't support it, but since SQLite is single-node, standard locking is fine.
	if !tm.db.Provider.IsSQLite() {
		query += " FOR UPDATE SKIP LOCKED"
	}

	err = tx.QueryRow(ctx, query).Scan(
		&task.ID, &task.MissionID, &task.Title, &description, &assignedAgentID,
		&task.Status, &task.Priority, &task.CreatedAt, &task.UpdatedAt,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" {
			return nil, nil // No pending tasks
		}
		return nil, fmt.Errorf("query task: %w", err)
	}

	task.Description = description.String
	task.AssignedAgentID = assignedAgentID.String

	// Update the task
	_, err = tx.Exec(ctx, `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = NOW()
		WHERE id = $2
	`, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("update task: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("commit tx: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = agentID
	task.UpdatedAt = time.Now()

	slog.Info("Claimed task", "taskID", task.ID, "agentID", agentID)

	return &task, nil
}

func (tm *TaskManager) CreateTask(ctx context.Context, missionID, title, description, priority string) (*SharedTask, error) {
	query := `
		INSERT INTO shared_tasks (mission_id, title, description, priority)
		VALUES ($1, $2, $3, $4)
		RETURNING id, status, created_at, updated_at
	`
	// sqlite doesn't always support RETURNING fully in modernc without specific setup, let's use standard insert then query if sqlite
	if tm.db.Provider.IsSQLite() {
		id := uuid.New().String()
		_, err := tm.db.Exec(ctx, `
			INSERT INTO shared_tasks (id, mission_id, title, description, priority)
			VALUES ($1, $2, $3, $4, $5)
		`, id, missionID, title, description, priority)
		if err != nil {
			return nil, fmt.Errorf("insert task sqlite: %w", err)
		}

		task := &SharedTask{
			ID:          id,
			MissionID:   missionID,
			Title:       title,
			Description: description,
			Priority:    priority,
			Status:      "PENDING",
			CreatedAt:   time.Now(),
			UpdatedAt:   time.Now(),
		}
		return task, nil
	}

	var task SharedTask
	task.MissionID = missionID
	task.Title = title
	task.Description = description
	task.Priority = priority

	err := tm.db.QueryRow(ctx, query, missionID, title, description, priority).Scan(
		&task.ID, &task.Status, &task.CreatedAt, &task.UpdatedAt,
	)
	if err != nil {
		return nil, fmt.Errorf("insert task postgres: %w", err)
	}

	return &task, nil
}
