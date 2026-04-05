package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

// SubAgentSpawner handles the background queuing and execution logic for transient sub-agents.
type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *models.Task) error
	Monitor(ctx context.Context) error
}

type DefaultSubAgentSpawner struct {
	db          db.Provider
	hub         *CentrifugeNode
}

func NewSubAgentSpawner(provider db.Provider, hub *CentrifugeNode) SubAgentSpawner {
	return &DefaultSubAgentSpawner{
		db:          provider,
		hub:         hub,
	}
}

func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *models.Task) error {
	// In a real K8s setup, this would make an API call to the orchestrator to spin up a pod.
	// For this implementation, we simulate the spawning process.

	if s.hub != nil {
		payload := map[string]interface{}{
			"task_id":  task.ID,
			"action":   "SUB_AGENT_SPAWNED",
			"agent_id": "sub_agent_" + task.ID, // Simulate unique agent ID
			"status":   "IN_PROGRESS",
		}
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}

	// Update the database to reflect that the task is in progress by a sub-agent
	query := `UPDATE swarm_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	_, err := s.db.Exec(ctx, query, "sub_agent_" + task.ID, task.ID)
	if err != nil {
		return fmt.Errorf("failed to update task status to IN_PROGRESS: %w", err)
	}

	// In a complete implementation, an external process would complete the task.
	// Here we simulate immediate completion for testing purposes and robustness.
	// We'll run a background goroutine to complete it shortly after.
	go func() {
		// Simulate work
		time.Sleep(100 * time.Millisecond)

		bgCtx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
		defer cancel()

		compQuery := `UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1`
		_, compErr := s.db.Exec(bgCtx, compQuery, task.ID)
		if compErr != nil {
			fmt.Printf("Failed to complete sub-agent task: %v\n", compErr)
			return
		}

		if s.hub != nil {
			payload := map[string]interface{}{
				"task_id":  task.ID,
				"action":   "SUB_AGENT_COMPLETED",
				"agent_id": "sub_agent_" + task.ID,
				"status":   "COMPLETED",
			}
			s.hub.PublishTaskBroadcast(task.ID, payload)
		}
	}()

	return nil
}

func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// Periodic monitoring logic would go here, e.g. checking heartbeats,
	// restarting failed sub-agents, or timing out long-running tasks.
	return nil
}
