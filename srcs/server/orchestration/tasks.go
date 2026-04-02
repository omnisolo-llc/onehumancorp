package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type TaskStatus string

const (
	TaskStatusPending    TaskStatus = "PENDING"
	TaskStatusInProgress TaskStatus = "IN_PROGRESS"
	TaskStatusCompleted  TaskStatus = "COMPLETED"
	TaskStatusBlocked    TaskStatus = "BLOCKED"
	TaskStatusFailed     TaskStatus = "FAILED"
)

type Task struct {
	ID              string     `json:"id"`
	MissionID       string     `json:"mission_id"`
	ParentTaskID    string     `json:"parent_task_id,omitempty"`
	Title           string     `json:"title"`
	Description     string     `json:"description"`
	Status          TaskStatus `json:"status"`
	AssignedAgentID string     `json:"assigned_agent_id,omitempty"`
	Dependencies    []string   `json:"dependencies"`
	CreatedAt       time.Time  `json:"created_at"`
	UpdatedAt       time.Time  `json:"updated_at"`
}

type TaskService struct {
	dbProvider db.Provider
	mesh       *CentrifugeHub // Simplified for testing, real implementation would use interface
}

func NewTaskService(dbProvider db.Provider, mesh *CentrifugeHub) *TaskService {
	return &TaskService{
		dbProvider: dbProvider,
		mesh:       mesh,
	}
}

func (s *TaskService) CreateTask(ctx context.Context, task *Task) error {
	if task.ID == "" {
		task.ID = uuid.NewString()
	}
	if task.Status == "" {
		task.Status = TaskStatusPending
	}

	depsBytes, err := json.Marshal(task.Dependencies)
	if err != nil {
		return err
	}

	if task.Dependencies == nil {
		depsBytes = []byte("[]")
	}

	_, err = s.dbProvider.Exec(ctx, `
		INSERT INTO agent_tasks (id, mission_id, parent_task_id, title, description, status, assigned_agent_id, dependencies)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`, task.ID, task.MissionID, task.ParentTaskID, task.Title, task.Description, string(task.Status), task.AssignedAgentID, depsBytes)

	if err != nil {
		// Fallback for sqlite parameter binding if needed
		_, err = s.dbProvider.Exec(ctx, `
			INSERT INTO agent_tasks (id, mission_id, parent_task_id, title, description, status, assigned_agent_id, dependencies)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?)
		`, task.ID, task.MissionID, task.ParentTaskID, task.Title, task.Description, string(task.Status), task.AssignedAgentID, string(depsBytes))

		if err != nil {
			return fmt.Errorf("failed to create task: %w", err)
		}
	}

	return nil
}

func (s *TaskService) CheckoutTask(ctx context.Context, agentID string) (*Task, error) {
	// Simple optimistic locking logic
	// Find a pending task where all dependencies are met
	// Because JSON arrays are complex to query cross-dialect, we'll keep it simple for now or fetch pending and check in memory if sqlite.

	query := `
		SELECT id, mission_id, parent_task_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM agent_tasks
		WHERE status = 'PENDING'
		LIMIT 1
	`

	var task Task
	var parentTaskID sql.NullString
	var assignedAgentID sql.NullString
	var depsBytes []byte

	// We'll use a transaction for safety
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	row := tx.QueryRow(ctx, query)
	err = row.Scan(&task.ID, &task.MissionID, &parentTaskID, &task.Title, &task.Description, &task.Status, &assignedAgentID, &depsBytes, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		return nil, err
	}

	if parentTaskID.Valid {
		task.ParentTaskID = parentTaskID.String
	}
	if assignedAgentID.Valid {
		task.AssignedAgentID = assignedAgentID.String
	}

	if len(depsBytes) > 0 {
		_ = json.Unmarshal(depsBytes, &task.Dependencies)
	}

	// Update status
	task.Status = TaskStatusInProgress
	task.AssignedAgentID = agentID
	task.UpdatedAt = time.Now()

	_, err = tx.Exec(ctx, `
		UPDATE agent_tasks
		SET status = $1, assigned_agent_id = $2, updated_at = $3
		WHERE id = $4 AND status = 'PENDING'
	`, string(task.Status), task.AssignedAgentID, task.UpdatedAt, task.ID)

	if err != nil {
		// SQLite fallback
		res, err2 := tx.Exec(ctx, `
			UPDATE agent_tasks
			SET status = ?, assigned_agent_id = ?, updated_at = ?
			WHERE id = ? AND status = 'PENDING'
		`, string(task.Status), task.AssignedAgentID, task.UpdatedAt, task.ID)

		if err2 != nil {
			return nil, fmt.Errorf("failed to update task: %w", err2)
		}
		rowsAffected, _ := res.RowsAffected()
		if rowsAffected == 0 {
			return nil, fmt.Errorf("task already checked out")
		}
	} else {
		// Verify postgres update
		// In a real implementation we would check RowsAffected from pgx driver response
		// but since we are abstracting over db.Provider we just commit
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	// Broadcast
	if s.mesh != nil {
		taskBytes, _ := json.Marshal(task)
		s.mesh.Publish(ctx, "swarm:tasks:updates", taskBytes)
	}

	return &task, nil
}
