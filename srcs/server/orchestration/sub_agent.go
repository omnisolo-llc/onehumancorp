package orchestration

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// SubAgentSpawner defines the interface for spawning and monitoring sub-agents.
type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *SharedTask) error
	Monitor(ctx context.Context) error
}

type DefaultSubAgentSpawner struct {
	hub Hub
	tm  *TaskManager
}

func NewDefaultSubAgentSpawner(hub Hub, tm *TaskManager) *DefaultSubAgentSpawner {
	return &DefaultSubAgentSpawner{
		hub: hub,
		tm:  tm,
	}
}

func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *SharedTask) error {
	slog.Info("Spawning sub-agent", "task_id", task.ID, "parent_plan_id", task.ParentPlanID)

	// Broadcast SUB_AGENT_SPAWNED event via Teammate Mesh
	if s.hub != nil {
		payload := map[string]interface{}{
			"task_id":        task.ID,
			"action":         "SUB_AGENT_SPAWNED",
			"agent_id":       "spawner-worker",
			"status":         "IN_PROGRESS",
			"parent_plan_id": task.ParentPlanID,
		}
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}

	// Simulate work asynchronously
	go func() {
		// Mock sub-agent execution delay
		time.Sleep(500 * time.Millisecond)

		// Create a context with claims for the completion operation
		completionCtx := auth.ContextWithClaims(context.Background(), &auth.Claims{OrganizationID: task.OrganizationID})

		err := s.tm.CompleteTask(completionCtx, task.ID, "spawner-worker")
		if err != nil {
			slog.Error("Failed to complete sub-agent task", "task_id", task.ID, "err", err)
			return
		}

		slog.Info("Sub-agent completed task", "task_id", task.ID)

		// The CompleteTask method already broadcasts COMPLETE, but we can also broadcast SUB_AGENT_COMPLETED if needed
		if s.hub != nil {
			payload := map[string]interface{}{
				"task_id":        task.ID,
				"action":         "SUB_AGENT_COMPLETED",
				"agent_id":       "spawner-worker",
				"status":         "COMPLETED",
				"parent_plan_id": task.ParentPlanID,
			}
			s.hub.PublishTaskBroadcast(task.ID, payload)
		}
	}()

	return nil
}

func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// Periodic heartbeat monitoring of sub-agents could be implemented here
	return nil
}
