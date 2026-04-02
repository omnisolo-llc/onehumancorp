package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
	"github.com/google/uuid"
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
	db          db.Provider
	redisClient rueidis.Client
	meshNode    *CentrifugeNode
}

// NewTaskManager creates a new TaskManager.
func NewTaskManager(provider db.Provider, meshNode *CentrifugeNode) *TaskManager {
	tm := &TaskManager{db: provider, meshNode: meshNode}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" {
			c, err := rueidis.NewClient(rueidis.ClientOption{
				InitAddress: []string{redisURL},
			})
			if err == nil {
				tm.redisClient = c
			}
		}
	}
	return tm
}

// CreateTask creates a new shared task.
func (tm *TaskManager) CreateTask(ctx context.Context, missionID, title, description, priority string) (*SharedTask, error) {
	if priority == "" {
		priority = "P2"
	}

	id := generateID()
	now := time.Now()
	task := SharedTask{
		ID:          id,
		MissionID:   missionID,
		Title:       title,
		Description: description,
		Status:      "PENDING",
		Priority:    priority,
		CreatedAt:   now,
		UpdatedAt:   now,
	}

	query := `
		INSERT INTO swarm_tasks (id, mission_id, title, description, priority, status, payload, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, '{}', $7, $8)
	`

	_, err := tm.db.Exec(ctx, query, id, missionID, title, description, priority, task.Status, task.CreatedAt, task.UpdatedAt)
	if err != nil {
		return nil, fmt.Errorf("failed to create task: %w", err)
	}

	if tm.meshNode != nil {
		tm.meshNode.PublishTaskBroadcast(EventTaskBroadcast{
			Type: "TASK_CREATED",
			Task: task,
		})
	}

	return &task, nil
}

// ClaimTask attempts to claim a specific PENDING task for the given agentID.
// It uses row-level locking (FOR UPDATE) in Postgres, and relies on SQLite's lock mechanism
// to prevent race conditions.
// In Multi-tenant cloud mode, it attempts to acquire a distributed Redis lock.
func (tm *TaskManager) ClaimTask(ctx context.Context, taskID, agentID string) (*SharedTask, error) {
	if tm.redisClient != nil {
		// Acquire Redis-backed distributed lock with 30s TTL
		lockKey := "lock:task:" + taskID
		cmd := tm.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Px(30000).Build()
		err := tm.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return nil, nil // Lock could not be acquired (task is locked)
			}
			return nil, fmt.Errorf("failed to acquire distributed lock: %w", err)
		}
	}

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
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		`
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Description, &task.Priority, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		// PostgreSQL with SKIP LOCKED
		query := `
			SELECT id, mission_id, title, description, priority, status, created_at, updated_at
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
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
		UPDATE swarm_tasks
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

	if tm.meshNode != nil {
		tm.meshNode.PublishTaskBroadcast(EventTaskBroadcast{
			Type: "TASK_CLAIMED",
			Task: task,
		})
	}

	return &task, nil
}

// CompleteTask marks a task as completed.
func (tm *TaskManager) CompleteTask(ctx context.Context, taskID, agentID string) error {
	query := `
		UPDATE swarm_tasks
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

	if tm.meshNode != nil {
		tm.meshNode.PublishTaskBroadcast(EventTaskBroadcast{
			Type: "TASK_COMPLETED",
			Task: SharedTask{ID: taskID, AssignedAgentID: agentID, Status: "COMPLETED"},
		})
	}

	return nil
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	return uuid.New().String()
}
