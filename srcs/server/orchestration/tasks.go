package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"time"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/go-redis/v9"
)

// SharedTask represents a shared task
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

// TaskManager manages shared tasks
type TaskManager struct {
	dbProvider  db.Provider
	redisClient *redis.Client
	isCloudMode bool
}

// NewTaskManager creates a new TaskManager
func NewTaskManager(dbProvider db.Provider, redisClient *redis.Client) *TaskManager {
	isCloud := os.Getenv("OHC_MULTITENANT") == "true"
	return &TaskManager{
		dbProvider:  dbProvider,
		redisClient: redisClient,
		isCloudMode: isCloud,
	}
}

// ClaimTask tries to claim a PENDING task
func (tm *TaskManager) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	if tm.isCloudMode && tm.redisClient != nil {
		return tm.claimTaskCloud(ctx, agentID)
	}
	return tm.claimTaskStandalone(ctx, agentID)
}

func (tm *TaskManager) claimTaskCloud(ctx context.Context, agentID string) (*SharedTask, error) {
	// First fetch a pending task
	var taskID string
	query := "SELECT id FROM shared_tasks WHERE status = 'PENDING' ORDER BY priority ASC, created_at ASC LIMIT 1"
	err := tm.dbProvider.QueryRow(ctx, query).Scan(&taskID)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, errors.New("no pending tasks")
		}
		return nil, err
	}

	// Try to acquire distributed lock
	lockKey := "task_lock:" + taskID
	acquired, err := tm.redisClient.SetNX(ctx, lockKey, agentID, 10*time.Second).Result()
	if err != nil {
		return nil, err
	}
	if !acquired {
		return nil, errors.New("could not acquire lock for task")
	}

	// Update task status
	updateQuery := "UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = NOW() WHERE id = $2 AND status = 'PENDING' RETURNING id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at"

	// Translate query for sqlite if using local test but multitenant set (fallback edge cases)
	if tm.dbProvider.IsSQLite() {
		updateQuery = "UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND status = 'PENDING' RETURNING id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at"
	}

	row := tm.dbProvider.QueryRow(ctx, updateQuery, agentID, taskID)

	task := &SharedTask{}
	err = row.Scan(&task.ID, &task.MissionID, &task.Title, &task.Description, &task.AssignedAgentID, &task.Status, &task.Priority, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		// Unlock if failed to claim
		tm.redisClient.Del(ctx, lockKey)
		return nil, err
	}

	return task, nil
}

func (tm *TaskManager) claimTaskStandalone(ctx context.Context, agentID string) (*SharedTask, error) {
	// Simple query and update without row lock since SQLite handles concurrent writes carefully or fails quickly.
	// We'll use a standard update returning pattern.
	query := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM shared_tasks
			WHERE status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		)
		RETURNING id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at
	`
	if tm.dbProvider.IsSQLite() {
		query = `
			UPDATE shared_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM shared_tasks
				WHERE status = 'PENDING'
				ORDER BY priority ASC, created_at ASC
				LIMIT 1
			)
			RETURNING id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at
		`
	}

	task := &SharedTask{}
	err := tm.dbProvider.QueryRow(ctx, query, agentID).Scan(&task.ID, &task.MissionID, &task.Title, &task.Description, &task.AssignedAgentID, &task.Status, &task.Priority, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, errors.New("no pending tasks")
		}
		return nil, err
	}

	return task, nil
}

// AddTask creates a new task
func (tm *TaskManager) AddTask(ctx context.Context, missionID, title, description, priority string) (*SharedTask, error) {
	query := "INSERT INTO shared_tasks (mission_id, title, description, priority) VALUES ($1, $2, $3, $4) RETURNING id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at"

	if tm.dbProvider.IsSQLite() {
		// SQLite might use different gen_random_uuid depending on extensions, but custom dbProvider handles translations.
		// Use manual gen uuid or assume dbProvider schema handles it (we assume schema translates it correctly or defaults work).
		query = "INSERT INTO shared_tasks (mission_id, title, description, priority, id) VALUES (?, ?, ?, ?, hex(randomblob(16))) RETURNING id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at"
	}

	task := &SharedTask{}
	var assignedAgentID sql.NullString
	err := tm.dbProvider.QueryRow(ctx, query, missionID, title, description, priority).Scan(&task.ID, &task.MissionID, &task.Title, &task.Description, &assignedAgentID, &task.Status, &task.Priority, &task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		return nil, err
	}

	if assignedAgentID.Valid {
		task.AssignedAgentID = assignedAgentID.String
	}

	return task, nil
}

// CompleteTask marks a task as COMPLETED
func (tm *TaskManager) CompleteTask(ctx context.Context, taskID string) error {
	query := "UPDATE shared_tasks SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1"
	if tm.dbProvider.IsSQLite() {
		query = "UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?"
	}

	_, err := tm.dbProvider.Exec(ctx, query, taskID)
	return err
}
