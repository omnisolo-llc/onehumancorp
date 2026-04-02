package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

type SwarmTask struct {
	ID              string          `json:"id"`
	MissionID       string          `json:"mission_id"`
	Title           string          `json:"title"`
	Status          string          `json:"status"`
	AssignedAgentID *string         `json:"assigned_agent_id"`
	LockedUntil     *time.Time      `json:"locked_until"`
	Payload         json.RawMessage `json:"payload"`
	CreatedAt       time.Time       `json:"created_at"`
	UpdatedAt       time.Time       `json:"updated_at"`
}

type TaskManager struct {
	db             *db.DB
	redis          rueidis.Client
	centrifugeNode *CentrifugeNode
}

func NewTaskManager(db *db.DB, redisClient rueidis.Client, centrifugeNode *CentrifugeNode) *TaskManager {
	return &TaskManager{
		db:             db,
		redis:          redisClient,
		centrifugeNode: centrifugeNode,
	}
}

// CreateTask adds a new task to the swarm.
func (m *TaskManager) CreateTask(ctx context.Context, task *SwarmTask) (*SwarmTask, error) {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}

	payloadStr, err := json.Marshal(task.Payload)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal payload: %w", err)
	}

	query := `
		INSERT INTO swarm_tasks (id, mission_id, title, status, payload)
		VALUES ($1, $2, $3, $4, $5)
	`

	if m.db.IsSQLite() {
		query = `
			INSERT INTO swarm_tasks (id, mission_id, title, status, payload)
			VALUES (?, ?, ?, ?, ?)
		`
	}

	_, err = m.db.Exec(ctx, query, task.ID, task.MissionID, task.Title, "PENDING", payloadStr)
	if err != nil {
		return nil, fmt.Errorf("failed to insert task: %w", err)
	}

	task.Status = "PENDING"

	if m.centrifugeNode != nil {
		m.centrifugeNode.PublishTaskBroadcast(task.ID, map[string]string{
			"status": "PENDING",
			"title": task.Title,
		})
	}

	return task, nil
}

// ClaimTask attempts to atomically claim a task.
func (m *TaskManager) ClaimTask(ctx context.Context, taskID, agentID string) (bool, error) {
	if m.redis != nil {
		// Redis-backed distributed lock with 30s TTL
		lockKey := "lock:task:" + taskID

		// SetNX equivalent in rueidis
		resp := m.redis.Do(ctx, m.redis.B().Set().Key(lockKey).Value(agentID).Nx().ExSeconds(30).Build())
		if err := resp.Error(); err != nil {
			if rueidis.IsRedisNil(err) {
				return false, nil // Lock not acquired
			}
			return false, fmt.Errorf("redis error claiming task: %w", err)
		}
	}

	// Update DB to ensure only one agent claims the PENDING task
	query := `
		UPDATE swarm_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = NOW()
		WHERE id = $2 AND status = 'PENDING'
	`
	if m.db.IsSQLite() {
		query = `
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP
			WHERE id = ? AND status = 'PENDING'
		`
	}

	rowsAffected, err := m.db.Exec(ctx, query, agentID, taskID)
	if err != nil {
		return false, fmt.Errorf("failed to claim task in db: %w", err)
	}

	if rowsAffected > 0 && m.centrifugeNode != nil {
		m.centrifugeNode.PublishTaskBroadcast(taskID, map[string]string{
			"status": "IN_PROGRESS",
			"agent_id": agentID,
		})
	}

	return rowsAffected > 0, nil
}

// CompleteTask marks a task as completed.
func (m *TaskManager) CompleteTask(ctx context.Context, taskID string, missionID string) error {
	query := `
		UPDATE swarm_tasks
		SET status = 'COMPLETED', updated_at = NOW()
		WHERE id = $1
	`
	if m.db.IsSQLite() {
		query = `
			UPDATE swarm_tasks
			SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
			WHERE id = ?
		`
	}

	_, err := m.db.Exec(ctx, query, taskID)
	if err != nil {
		return fmt.Errorf("failed to complete task: %w", err)
	}

	telemetry.RecordSwarmTaskCompleted(ctx, missionID)

	if m.centrifugeNode != nil {
		m.centrifugeNode.PublishTaskBroadcast(taskID, map[string]string{
			"status": "COMPLETED",
		})
	}

	return nil
}
