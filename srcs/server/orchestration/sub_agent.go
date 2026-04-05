package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"sync"
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
	mu   sync.Mutex
}

func NewSubAgentSpawner(provider db.Provider, hub *CentrifugeNode, mesh TeammateMesh) SubAgentSpawner {
	return &DefaultSubAgentSpawner{
		db:   provider,
		mesh: mesh,
		hub:  hub,
	}
}

func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *models.Task) error {
	// Emit SUB_AGENT_SPAWNED via mesh or hub
	s.broadcastLifecycleEvent(ctx, task, "SUB_AGENT_SPAWNED")

	// Simulate background work execution
	go func() {
		bgCtx, cancel := context.WithTimeout(context.Background(), 5*time.Minute)
		defer cancel()

		slog.Info("SubAgentSpawner: executing task", "taskID", task.ID)

		// Wait a bit to simulate work
		select {
		case <-time.After(100 * time.Millisecond): // Quick for tests
		case <-bgCtx.Done():
			slog.Warn("SubAgentSpawner: task timed out", "taskID", task.ID)
			return
		}

		// Mark task as completed
		err := s.completeTask(bgCtx, task.ID, "SUB_AGENT_WORKER", "Sub-agent work finished successfully")
		if err != nil {
			slog.Error("SubAgentSpawner: failed to complete task", "taskID", task.ID, "error", err)
			return
		}

		// Emit SUB_AGENT_COMPLETED
		s.broadcastLifecycleEvent(bgCtx, task, "SUB_AGENT_COMPLETED")
	}()

	return nil
}

func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// Not fully implemented: monitoring heartbeats
	return nil
}

func (s *DefaultSubAgentSpawner) broadcastLifecycleEvent(ctx context.Context, task *models.Task, action string) {
	if s.mesh != nil {
		_ = s.mesh.BroadcastTask(ctx, Task{
			AgentID: "SUB_AGENT_WORKER",
			Action:  action,
			Status:  task.Status,
			TaskID:  task.ID,
		})
	} else if s.hub != nil {
		payload := map[string]interface{}{
			"task_id":  task.ID,
			"action":   action,
			"agent_id": "SUB_AGENT_WORKER",
			"status":   task.Status,
		}
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}
}

// Minimal implementation to allow spawner to complete a task itself in the background
func (s *DefaultSubAgentSpawner) completeTask(ctx context.Context, taskID string, agentID string, result string) error {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND status = 'IN_PROGRESS'", taskID)
	if err != nil {
		return err
	}

	// We skip DAG dependency unblocking in this simplistic complete logic for sub-agents,
	// relying on TaskOrchestrator for complex flow. For full KAIROS it should probably call orchestrator.CompleteTask.

	return tx.Commit(ctx)
}
