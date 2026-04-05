package orchestration

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

// SubAgentSpawner handles the background queuing logic to spawn isolated, transient sub-agents.
type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *models.Task) error
	Monitor(ctx context.Context) error
}

type DefaultSubAgentSpawner struct {
	db   db.Provider
	hub  *CentrifugeNode
	mesh TeammateMesh
}

func NewDefaultSubAgentSpawner(provider db.Provider, hub *CentrifugeNode, mesh TeammateMesh) *DefaultSubAgentSpawner {
	return &DefaultSubAgentSpawner{
		db:   provider,
		hub:  hub,
		mesh: mesh,
	}
}

func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *models.Task) error {
	// Broadcast SUB_AGENT_SPAWNED
	if s.mesh != nil {
		_ = s.mesh.BroadcastTask(ctx, Task{
			AgentID: "sub-agent-spawner",
			Action:  "SUB_AGENT_SPAWNED",
			Status:  task.Status,
			TaskID:  task.ID,
		})
	} else if s.hub != nil {
		payload := map[string]interface{}{
			"task_id": task.ID,
			"action":  "SUB_AGENT_SPAWNED",
			"status":  task.Status,
		}
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}

	// In a real system, this would spin up a K8s pod or a dedicated worker.
	// For our standalone/local execution, we'll spawn a goroutine to simulate the sub-agent.
	go s.runSubAgent(context.Background(), task)

	return nil
}

func (s *DefaultSubAgentSpawner) runSubAgent(ctx context.Context, task *models.Task) {
	// Simulate work and exponential backoff
	backoff := 100 * time.Millisecond
	maxRetries := 3

	for i := 0; i < maxRetries; i++ {
		// Write heartbeat
		s.writeHeartbeat(task.ID)

		// Simulate some work
		select {
		case <-ctx.Done():
			return
		case <-time.After(backoff):
			// Proceed
		}

		backoff *= 2
	}

	// Broadcast SUB_AGENT_COMPLETED
	if s.mesh != nil {
		_ = s.mesh.BroadcastTask(ctx, Task{
			AgentID: "sub-agent-spawner",
			Action:  "SUB_AGENT_COMPLETED",
			Status:  "COMPLETED",
			TaskID:  task.ID,
		})
	} else if s.hub != nil {
		payload := map[string]interface{}{
			"task_id": task.ID,
			"action":  "SUB_AGENT_COMPLETED",
			"status":  "COMPLETED",
		}
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}
}

func (s *DefaultSubAgentSpawner) writeHeartbeat(taskID string) {
	// Write to .agent-task/status/{timestamp}.yml
	statusDir := ".agent-task/status"
	if err := os.MkdirAll(statusDir, 0755); err != nil {
		return
	}

	timestamp := time.Now().UTC().Format(time.RFC3339)
	filename := filepath.Join(statusDir, fmt.Sprintf("%s_%s.yml", timestamp, taskID))

	content := fmt.Sprintf("task_id: %s\nstatus: HEARTBEAT\ntimestamp: %s\n", taskID, timestamp)
	_ = os.WriteFile(filename, []byte(content), 0644)
}

func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// Not fully implemented for this mission, but satisfies interface.
	// In a complete implementation, this would track orphaned tasks and retry them.
	return nil
}
