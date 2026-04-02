package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// TaskStatus represents the lifecycle state of a swarm task.
type TaskStatus string

const (
	TaskStatusPending    TaskStatus = "PENDING"
	TaskStatusInProgress TaskStatus = "IN_PROGRESS"
	TaskStatusCompleted  TaskStatus = "COMPLETED"
	TaskStatusFailed     TaskStatus = "FAILED"
)

// SwarmTask represents a task in the shared task list.
type SwarmTask struct {
	ID              string     `json:"id"`
	MissionID       string     `json:"mission_id"`
	Title           string     `json:"title"`
	Status          TaskStatus `json:"status"`
	AssignedAgentID string     `json:"assigned_agent_id"`
	LockedUntil     time.Time  `json:"locked_until"`
	Payload         string     `json:"payload"`
}

// SwarmLongTermMemory represents a consolidated long-term memory for autoDream.
type SwarmLongTermMemory struct {
	ID        string    `json:"id"`
	Topic     string    `json:"topic"`
	Summary   string    `json:"summary"`
	Embedding []float32 `json:"embedding"`
	CreatedAt time.Time `json:"created_at"`
}

// TaskQueueManager handles claiming tasks from the shared swarm queue.
type TaskQueueManager struct {
	db          db.Provider
	redisClient rueidis.Client
}

// NewTaskQueueManager creates a TaskQueueManager.
func NewTaskQueueManager(db db.Provider, redisClient rueidis.Client) *TaskQueueManager {
	return &TaskQueueManager{
		db:          db,
		redisClient: redisClient,
	}
}

// ClaimTask attempts to assign a task to an agent safely preventing race conditions.
func (m *TaskQueueManager) ClaimTask(ctx context.Context, taskID, agentID string) error {
	if m.redisClient != nil {
		// Redis distributed lock
		lockKey := "lock:task:" + taskID
		cmd := m.redisClient.B().Setnx().Key(lockKey).Value(agentID).Build()
		// No TTL on setnx build natively in rueidis without multi command, let's use SET lockKey agentID EX 30 NX
		cmd2 := m.redisClient.B().Set().Key(lockKey).Value(agentID).Ex(30 * time.Second).Nx().Build()

		res := m.redisClient.Do(ctx, cmd2)
		if res.Error() != nil {
			return fmt.Errorf("redis lock failed: %w", res.Error())
		}

		// If res is nil, it means it wasn't set (because of NX)
		if res.IsNil() {
			return errors.New("task already locked by another agent")
		}

		// Proceed to update the DB
		_, err := m.db.Exec(ctx,
			"UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = $2 WHERE id = $3 AND status = 'PENDING'",
			agentID, time.Now().Add(30*time.Second), taskID)
		if err != nil {
			return fmt.Errorf("failed to update task status: %w", err)
		}

		return nil
	}

	// Standalone mode: SQLite transactional lock
	if m.db.IsSQLite() {
		tx, err := m.db.Begin(ctx)
		if err != nil {
			return err
		}
		defer func() { _ = tx.Rollback(ctx) }()

		var status string
		err = tx.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = ?", taskID).Scan(&status)
		if err != nil {
			return fmt.Errorf("task not found: %w", err)
		}

		if status != string(TaskStatusPending) {
			return errors.New("task is not pending")
		}

		_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = ?, locked_until = ? WHERE id = ?", agentID, time.Now().Add(30*time.Second), taskID)
		if err != nil {
			return fmt.Errorf("failed to assign task: %w", err)
		}

		return tx.Commit(ctx)
	}

	// Postgres direct lock if no Redis (fallback)
	tx, err := m.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer func() { _ = tx.Rollback(ctx) }()

	var status string
	err = tx.QueryRow(ctx, "SELECT status FROM swarm_tasks WHERE id = $1 FOR UPDATE NOWAIT", taskID).Scan(&status)
	if err != nil {
		return fmt.Errorf("failed to lock task row (possibly locked by another process): %w", err)
	}

	if status != string(TaskStatusPending) {
		return errors.New("task is not pending")
	}

	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, locked_until = $2 WHERE id = $3", agentID, time.Now().Add(30*time.Second), taskID)
	if err != nil {
		return fmt.Errorf("failed to update task status: %w", err)
	}

	return tx.Commit(ctx)
}

// CompleteTask marks a task as completed.
func (m *TaskQueueManager) CompleteTask(ctx context.Context, taskID, agentID string) error {
	var rowsAffected int64
	var err error
	if m.db.IsSQLite() {
		rowsAffected, err = m.db.Exec(ctx, "UPDATE swarm_tasks SET status = 'COMPLETED' WHERE id = ? AND assigned_agent_id = ?", taskID, agentID)
	} else {
		rowsAffected, err = m.db.Exec(ctx, "UPDATE swarm_tasks SET status = 'COMPLETED' WHERE id = $1 AND assigned_agent_id = $2", taskID, agentID)
	}

	if err != nil {
		return err
	}
	if rowsAffected == 0 {
		return errors.New("task not found or not assigned to agent")
	}

	// Observability
	telemetry.RecordSwarmTaskCompleted(ctx, taskID, agentID)
	return nil
}

// AddTask adds a new task to the shared swarm queue.
func (m *TaskQueueManager) AddTask(ctx context.Context, missionID, title, payload string) (string, error) {
	id := uuid.New().String()
	var err error
	if m.db.IsSQLite() {
		_, err = m.db.Exec(ctx,
			"INSERT INTO swarm_tasks (id, mission_id, title, status, payload, created_at, updated_at) VALUES (?, ?, ?, 'PENDING', ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
			id, missionID, title, payload)
	} else {
		_, err = m.db.Exec(ctx,
			"INSERT INTO swarm_tasks (id, mission_id, title, status, payload, created_at, updated_at) VALUES ($1, $2, $3, 'PENDING', $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
			id, missionID, title, payload)
	}
	return id, err
}
