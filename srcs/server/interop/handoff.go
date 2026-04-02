package interop

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// EventTaskBroadcast represents a task broadcasted across the Teammate Mesh.
type EventTaskBroadcast struct {
	ID              string    `json:"id"`
	MissionID       string    `json:"mission_id"`
	Title           string    `json:"title"`
	Description     string    `json:"description"`
	AssignedAgentID string    `json:"assigned_agent_id,omitempty"`
	Status          string    `json:"status"`
	Priority        string    `json:"priority"`
	Timestamp       time.Time `json:"timestamp"`
}

// CrossModeHandoff manages the distribution of tasks in either Standalone or Cloud mode.
type CrossModeHandoff struct {
	db    db.Provider
	redis rueidis.Client
}

// NewCrossModeHandoff initializes a new handoff manager.
func NewCrossModeHandoff(db db.Provider, redis rueidis.Client) *CrossModeHandoff {
	return &CrossModeHandoff{
		db:    db,
		redis: redis,
	}
}

// BroadcastTask broadcasts a task to the swarm.
// In both modes, it persists the task to the database.
// In Cloud Mode (when Redis is available), it publishes the task to the "mesh:tasks" channel.
func (h *CrossModeHandoff) BroadcastTask(ctx context.Context, task EventTaskBroadcast) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}
	if task.Timestamp.IsZero() {
		task.Timestamp = time.Now()
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}
	if task.Priority == "" {
		task.Priority = "P2"
	}

	// 1. Persist to shared_tasks
	query := `
		INSERT INTO shared_tasks (id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
	`
	if h.db.IsSQLite() {
		query = `
			INSERT INTO shared_tasks (id, mission_id, title, description, assigned_agent_id, status, priority, created_at, updated_at)
			VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
		`
	}

	_, err := h.db.Exec(ctx, query,
		task.ID,
		task.MissionID,
		task.Title,
		task.Description,
		task.AssignedAgentID,
		task.Status,
		task.Priority,
		task.Timestamp,
		task.Timestamp, // Added 9th argument here!
	)
	if err != nil {
		return fmt.Errorf("failed to persist task to db: %w", err)
	}

	slog.Info("CrossModeHandoff: Task persisted to shared_tasks", "task_id", task.ID, "mission_id", task.MissionID)

	// 2. Publish to Redis Pub/Sub if in Cloud mode
	if h.redis != nil {
		payload, err := json.Marshal(task)
		if err != nil {
			return fmt.Errorf("failed to marshal task for broadcast: %w", err)
		}

		cmd := h.redis.B().Publish().Channel("mesh:tasks").Message(string(payload)).Build()
		err = h.redis.Do(ctx, cmd).Error()
		if err != nil {
			return fmt.Errorf("failed to publish task to redis: %w", err)
		}
		slog.Info("CrossModeHandoff: Task broadcasted via Redis", "task_id", task.ID, "channel", "mesh:tasks")
	} else {
		// In standalone mode, we rely on local polling or internal IPC which could read the DB.
		slog.Debug("CrossModeHandoff: Running in standalone mode, task broadcast limited to DB persistence.")
	}

	return nil
}
