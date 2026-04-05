package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *models.Task) error
	Monitor(ctx context.Context) error
}

type DefaultSubAgentSpawner struct {
	db   db.Provider
	mesh TeammateMesh
	hub  *CentrifugeNode
}

func NewSubAgentSpawner(db db.Provider, mesh TeammateMesh, hub *CentrifugeNode) SubAgentSpawner {
	return &DefaultSubAgentSpawner{
		db:   db,
		mesh: mesh,
		hub:  hub,
	}
}

func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *models.Task) error {
	slog.Info("Spawning sub-agent", "taskID", task.ID)

	// Broadcast SUB_AGENT_SPAWNED
	s.broadcastLifecycle(ctx, task.ID, "SUB_AGENT_SPAWNED")

	// Simulate sub-agent execution synchronously for now, in a real env this spawns a K8s pod or background job.
	go func(t *models.Task) {
		bgCtx, cancel := context.WithTimeout(context.Background(), 5*time.Minute)
		defer cancel()

		// Simulate some work
		time.Sleep(100 * time.Millisecond)

		// Record heartbeat for observability
		s.recordHeartbeat(t.ID)

		// Broadcast SUB_AGENT_COMPLETED
		s.broadcastLifecycle(bgCtx, t.ID, "SUB_AGENT_COMPLETED")

	}(task)

	return nil
}

func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// A placeholder for monitoring running sub-agents if we implemented a tracking map or K8s informer
	return nil
}

func (s *DefaultSubAgentSpawner) broadcastLifecycle(ctx context.Context, taskID, action string) {
	if s.mesh != nil {
		_ = s.mesh.BroadcastTask(ctx, Task{
			AgentID: "sub-agent-system",
			Action:  action,
			Status:  action,
			TaskID:  taskID,
		})
	} else if s.hub != nil {
		payload := map[string]interface{}{
			"task_id":  taskID,
			"action":   action,
			"agent_id": "sub-agent-system",
			"status":   action,
		}
		s.hub.PublishTaskBroadcast(taskID, payload)
	}
}

func (s *DefaultSubAgentSpawner) recordHeartbeat(taskID string) {
	// Implementation would write to .agent-task/status/{timestamp}.yml
	// or standard health check endpoint depending on OHC-SIP
	slog.Debug("Recorded heartbeat for sub-agent", "taskID", taskID)
}
