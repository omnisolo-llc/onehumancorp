package orchestration

import (
	"context"
	"log/slog"
	"sync"
	"time"

	"github.com/onehumancorp/ohc/srcs/server/db"
)

// SubAgentSpawner handles the isolated, transient sub-agent spawning and monitoring.
type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *SharedTask) error
	Monitor(ctx context.Context) error
}

// DefaultSubAgentSpawner implements SubAgentSpawner.
type DefaultSubAgentSpawner struct {
	db       db.Provider
	hub      *CentrifugeNode
	mesh     TeammateMesh
	sipdb    db.Provider
	sem      chan struct{} // Used for SQLite concurrency limits
	wg       sync.WaitGroup
}

// NewSubAgentSpawner creates a new SubAgentSpawner.
func NewSubAgentSpawner(db db.Provider, hub *CentrifugeNode, mesh TeammateMesh, sipdb db.Provider) SubAgentSpawner {
	var sem chan struct{}
	if db.IsSQLite() {
		// Enforce concurrency limit in standalone mode
		sem = make(chan struct{}, 5) // Hard limit to prevent CPU exhaustion locally
	}
	return &DefaultSubAgentSpawner{
		db:    db,
		hub:   hub,
		mesh:  mesh,
		sipdb: sipdb,
		sem:   sem,
	}
}

func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *SharedTask) error {
	if s.sem != nil {
		s.sem <- struct{}{}
	}

	// 1. Broadcast Sub-Agent Spawned via Teammate Mesh
	s.broadcastLifecycleEvent(ctx, task, "SUB_AGENT_SPAWNED")

	s.wg.Add(1)
	go func() {
		defer s.wg.Done()
		if s.sem != nil {
			defer func() { <-s.sem }()
		}

		// Wait a bit to simulate work and context load.
		select {
		case <-ctx.Done():
			return
		case <-time.After(500 * time.Millisecond):
		}

		// Simulate exponential backoff for a random operation, then success
		maxRetries := 3
		retryDelay := 100 * time.Millisecond

		for attempt := 0; attempt < maxRetries; attempt++ {
			// Write an observability heartbeat to SIPDB if configured
			if s.sipdb != nil {
				_ = s.writeHeartbeat(ctx, task.ID, "RUNNING")
			}

			// Simulate sub-agent process
			select {
			case <-ctx.Done():
				return
			case <-time.After(100 * time.Millisecond):
			}

			// We assume success for this prototype sub-agent execution
			break
		}

		// Mark complete
		s.completeTask(ctx, task)
	}()

	return nil
}

func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// In a real system, Monitor might look for orphaned tasks and re-queue them.
	// We'll leave it as a no-op waiting mechanism for now, to satisfy interface.
	s.wg.Wait()
	return nil
}

func (s *DefaultSubAgentSpawner) completeTask(ctx context.Context, task *SharedTask) {
	// Broadcast Sub-Agent Completed
	s.broadcastLifecycleEvent(ctx, task, "SUB_AGENT_COMPLETED")

	// Update the database to reflect completion
	tx, err := s.db.Begin(ctx)
	if err != nil {
		slog.Error("SubAgentSpawner: Failed to begin tx", "error", err)
		return
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, "UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1", task.ID)
	if err != nil {
		slog.Error("SubAgentSpawner: Failed to update task status", "error", err)
		return
	}

	_ = tx.Commit(ctx)
}

func (s *DefaultSubAgentSpawner) broadcastLifecycleEvent(ctx context.Context, task *SharedTask, action string) {
	if s.mesh != nil {
		_ = s.mesh.BroadcastTask(ctx, Task{
			AgentID: "sub-agent-spawner",
			Action:  action,
			Status:  task.Status,
			TaskID:  task.ID,
		})
	} else if s.hub != nil {
		payload := map[string]interface{}{
			"task_id":  task.ID,
			"action":   action,
			"agent_id": "sub-agent-spawner",
			"status":   task.Status,
		}
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}
}

func (s *DefaultSubAgentSpawner) writeHeartbeat(ctx context.Context, taskID, status string) error {
	query := `INSERT INTO agent_heartbeats (id, task_id, status, created_at) VALUES ($1, $2, $3, CURRENT_TIMESTAMP)`
	id := generateID()
	_, err := s.sipdb.Exec(ctx, query, id, taskID, status)
	return err
}
