package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

type TaskStatus string

const (
	TaskStatusPending    TaskStatus = "PENDING"
	TaskStatusInProgress TaskStatus = "IN_PROGRESS"
	TaskStatusBlocked    TaskStatus = "BLOCKED"
	TaskStatusCompleted  TaskStatus = "COMPLETED"
	TaskStatusFailed     TaskStatus = "FAILED"
)

type SwarmTask struct {
	ID              string          `json:"id"`
	MissionID       string          `json:"mission_id"`
	ParentTaskID    *string         `json:"parent_task_id"`
	Title           string          `json:"title"`
	Description     *string         `json:"description"`
	Status          TaskStatus      `json:"status"`
	AssignedAgentID *string         `json:"assigned_agent_id"`
	Dependencies    []string        `json:"dependencies"`
	LockedUntil     *time.Time      `json:"locked_until"`
	Payload         json.RawMessage `json:"payload"`
	CreatedAt       time.Time       `json:"created_at"`
	UpdatedAt       time.Time       `json:"updated_at"`
}

type SwarmTaskStore struct {
	provider    db.Provider
	redisClient rueidis.Client
	hub         *Hub
}

func NewSwarmTaskStore(provider db.Provider, redisClient rueidis.Client, hub *Hub) *SwarmTaskStore {
	return &SwarmTaskStore{
		provider:    provider,
		redisClient: redisClient,
		hub:         hub,
	}
}

func (s *SwarmTaskStore) CreateTask(ctx context.Context, task *SwarmTask) error {
	deps, err := json.Marshal(task.Dependencies)
	if err != nil {
		return err
	}

	_, err = s.provider.Exec(ctx,
		`INSERT INTO swarm_tasks (mission_id, parent_task_id, title, description, status, assigned_agent_id, dependencies, locked_until, payload)
		 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)`,
		task.MissionID, task.ParentTaskID, task.Title, task.Description, task.Status, task.AssignedAgentID, deps, task.LockedUntil, task.Payload)
	return err
}

func (s *SwarmTaskStore) ClaimTask(ctx context.Context, taskID, agentID string) (bool, error) {
	// First, check distributed lock
	if s.redisClient != nil {
		cmd := s.redisClient.B().Set().Key("lock:task:" + taskID).Value(agentID).Nx().Px(30000).Build()
		// We set a 30 second TTL
		acquired, err := s.redisClient.Do(ctx, cmd).AsBool()
		if err != nil {
			return false, fmt.Errorf("redis lock err: %w", err)
		}
		if !acquired {
			return false, nil // Already claimed
		}

	}

	// Now try to claim in database atomically (optimistic locking essentially)
	res, err := s.provider.Exec(ctx,
		"UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND status = 'PENDING'",
		agentID, taskID)
	if err != nil {
		return false, err
	}

	if res == 0 {
		return false, nil // Could not claim or status was not PENDING
	}

	// Increment completed counter correctly
	telemetry.RecordSwarmTaskCompleted(ctx)

	// Broadcast through mesh
	s.broadcastTaskUpdate(ctx, taskID, "IN_PROGRESS", agentID)

	return true, nil
}

func (s *SwarmTaskStore) CompleteTask(ctx context.Context, taskID string) error {
	res, err := s.provider.Exec(ctx,
		"UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1", taskID)
	if err != nil {
		return err
	}
	if res > 0 {
		s.broadcastTaskUpdate(ctx, taskID, "COMPLETED", "")
	}
	return nil
}

func (s *SwarmTaskStore) broadcastTaskUpdate(ctx context.Context, taskID, status, agentID string) {
	// Create broadcast payload
	payload := map[string]interface{}{
		"type":    "TASK_BROADCAST",
		"task_id": taskID,
		"status":  status,
		"agent_id": agentID,
	}
	payloadBytes, _ := json.Marshal(payload)

	// Broadcast using Redis or local
	if s.redisClient != nil {
		cmd := s.redisClient.B().Publish().Channel("swarm:tasks:updates").Message(string(payloadBytes)).Build()
		s.redisClient.Do(ctx, cmd)
	}

	// If hub is set, publish internal message?
	if s.hub != nil && s.hub.CentrifugeNode() != nil {
		// Just reuse Centrifuge if available
		_ = s.hub.CentrifugeNode().PublishToChannel(ctx, "swarm:tasks:updates", payloadBytes)
	}
}
