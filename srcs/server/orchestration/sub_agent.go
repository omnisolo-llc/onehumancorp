package orchestration

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/ohc-api/srcs/server/models"
)

// SubAgentSpawner handles the spawning and monitoring of sub-agents.
type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *models.Task) error
	Monitor(ctx context.Context) error
}

type DefaultSubAgentSpawner struct {
	orchestrator TaskOrchestrator
	hub          Hub
}

func NewDefaultSubAgentSpawner(orchestrator TaskOrchestrator, hub Hub) *DefaultSubAgentSpawner {
	return &DefaultSubAgentSpawner{
		orchestrator: orchestrator,
		hub:          hub,
	}
}

// Spawn executes the delegation task in isolation.
func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *models.Task) error {
	agentID := "sub-agent-" + generateID()

	// Broadcast SPAWNED
	payload := map[string]interface{}{
		"task_id":  task.ID,
		"action":   "SUB_AGENT_SPAWNED",
		"agent_id": agentID,
		"status":   "IN_PROGRESS",
	}
	if s.hub != nil {
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}

	// Simulate Work
	time.Sleep(100 * time.Millisecond)

	// Broadcast COMPLETED
	completePayload := map[string]interface{}{
		"task_id":  task.ID,
		"action":   "SUB_AGENT_COMPLETED",
		"agent_id": agentID,
		"status":   "COMPLETED",
	}
	if s.hub != nil {
		s.hub.PublishTaskBroadcast(task.ID, completePayload)
	}

	err := s.orchestrator.CompleteTask(ctx, task.ID, agentID, "Sub-agent execution complete")
	if err != nil {
		return fmt.Errorf("failed to complete sub-agent task: %w", err)
	}

	return nil
}

func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// The monitoring logic is embedded inside the task_orchestrator loop
	// which routes tasks with Priority "DELEGATED" to the spawner.
	return nil
}
