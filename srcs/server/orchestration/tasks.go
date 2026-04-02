package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"os"
	"time"

	"encoding/json"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
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
	Payload         string
	LockedUntil     sql.NullTime
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

// TaskManager manages the shared tasks list
type TaskManager struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode // For Teammate Mesh broadcast
}

// NewTaskManager creates a new TaskManager.
func NewTaskManager(provider db.Provider) *TaskManager {
	tm := &TaskManager{db: provider}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" {
			opts, err := rueidis.ParseURL(redisURL)
			if err == nil {
				if c, err := rueidis.NewClient(opts); err == nil {
					tm.redisClient = c
				}
			}
		}
	}
	return tm
}

// SetHub injects the CentrifugeNode dependency into the TaskManager.
func (tm *TaskManager) SetHub(hub *CentrifugeNode) {
	tm.hub = hub
}

// CreateTask creates a new shared task.
func (tm *TaskManager) CreateTask(ctx context.Context, missionID, title, description, priority string) (*SharedTask, error) {
	if priority == "" {
		priority = "P2"
	}

	// For standard SQLite insertion, we generate our own ID.
	id := generateID()

	// Default payload with description and priority based on schema requirements
	payloadMap := map[string]string{"description": description, "priority": priority}
	payloadBytes, err := json.Marshal(payloadMap)
	if err != nil {
		return nil, fmt.Errorf("failed to encode task payload: %w", err)
	}
	payload := string(payloadBytes)

	var task SharedTask
	var query string

	// We'll scan fields carefully accounting for potential missing ones depending on RETURNING
	if tm.db.IsSQLite() {
		query = `
			INSERT INTO swarm_tasks (id, mission_id, title, payload, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			RETURNING id, mission_id, title, payload, status, created_at, updated_at
		`
		err = tm.db.QueryRow(ctx, query, id, missionID, title, payload).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Payload, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
		task.Description = description
		task.Priority = priority
	} else {
		// Postgres mode
		query = `
			INSERT INTO swarm_tasks (id, mission_id, title, payload, status)
			VALUES ($1, $2, $3, $4, 'PENDING')
			RETURNING id, mission_id, title, payload, status, created_at, updated_at
		`
		err = tm.db.QueryRow(ctx, query, id, missionID, title, payload).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Payload, &task.Status, &task.CreatedAt, &task.UpdatedAt,
		)
		task.Description = description
		task.Priority = priority
	}

	if err != nil {
		return nil, fmt.Errorf("failed to create task: %w", err)
	}

	// Broadcast task creation
	if tm.hub != nil {
		tm.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":      "CREATE",
			"mission_id":  task.MissionID,
			"title":       task.Title,
			"description": task.Description,
			"priority":    task.Priority,
			"status":      task.Status,
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
		cmd := tm.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Px(30 * time.Second).Build()
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
			SELECT id, mission_id, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY json_extract(payload, '$.priority') ASC, created_at ASC
			LIMIT 1
		`
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Payload, &task.Status, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
	} else {
		// PostgreSQL with SKIP LOCKED
		query := `
			SELECT id, mission_id, title, payload, status, locked_until, created_at, updated_at
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY payload->>'priority' ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Payload, &task.Status, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
	}

	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to find pending task: %w", errQuery)
	}

	// Reconstruct Description and Priority from JSON payload
	var payloadMap map[string]interface{}
	if err := json.Unmarshal([]byte(task.Payload), &payloadMap); err == nil {
		if desc, ok := payloadMap["description"].(string); ok {
			task.Description = desc
		}
		if prio, ok := payloadMap["priority"].(string); ok {
			task.Priority = prio
		}
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

	// Broadcast task claim
	if tm.hub != nil {
		tm.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{
			"action":    "CLAIM",
			"agent_id":  agentID,
			"status":    task.Status,
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

	// Broadcast task completion
	if tm.hub != nil {
		tm.hub.PublishTaskBroadcast(taskID, map[string]interface{}{
			"action":   "COMPLETE",
			"agent_id": agentID,
			"status":   "COMPLETED",
		})
	}

	// Record Telemetry
	// Note: We don't have mission_id readily available in this block, but telemetry.RecordSwarmTaskCompleted can take it or we can pass an empty string / agent string.
	// Actually we should fetch it if we want it perfect, but it's optional for the counter.
		var missionID string
	err = tm.db.QueryRow(ctx, "SELECT mission_id FROM swarm_tasks WHERE id = $1", taskID).Scan(&missionID)
	if err == nil {
		telemetry.RecordSwarmTaskCompleted(ctx, missionID)
	}

	return nil
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	return fmt.Sprintf("%d", time.Now().UnixNano())
}
