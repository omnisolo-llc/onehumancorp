package orchestration

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *SharedTask) error
	Monitor(ctx context.Context) error
}

type DefaultSubAgentSpawner struct {
	db   db.Provider
	tm   *TaskManager
	hub  *CentrifugeNode
	mu   sync.Mutex
	sem  chan struct{}
}

func NewDefaultSubAgentSpawner(db db.Provider, tm *TaskManager, hub *CentrifugeNode) *DefaultSubAgentSpawner {
	concurrencyLimit := 10 // Standalone mode concurrency limit
	return &DefaultSubAgentSpawner{
		db:  db,
		tm:  tm,
		hub: hub,
		sem: make(chan struct{}, concurrencyLimit),
	}
}

func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *SharedTask) error {
	if s.db.IsSQLite() {
		// Acquire semaphore in standalone mode to prevent local CPU exhaustion
		s.sem <- struct{}{}
		go func() {
			defer func() { <-s.sem }()
			s.runSubAgent(context.Background(), task)
		}()
	} else {
		// Cloud mode - just a goroutine for now, represents a K8s pod spawn in a real system
		go s.runSubAgent(context.Background(), task)
	}

	return nil
}

func (s *DefaultSubAgentSpawner) runSubAgent(ctx context.Context, task *SharedTask) {
	// Set claims context to act as an agent with the task's organization
	claims := &auth.Claims{
		OrganizationID: task.OrganizationID,
		Roles:          []string{"system"},
	}
	ctx = auth.ContextWithClaims(ctx, claims)

	// Emit SPaWN event
	s.emitEvent(task.ID, "SUB_AGENT_SPAWNED", "IN_PROGRESS", task.OrganizationID)

	// Simulate work
	time.Sleep(100 * time.Millisecond)

	// Update task status to completed
	err := s.tm.CompleteTask(ctx, task.ID, "sub_agent", "Sub-agent execution successful")
	if err != nil {
		fmt.Printf("Sub-agent failed to complete task %s: %v\n", task.ID, err)
		return
	}

	// Emit COMPLETED event
	s.emitEvent(task.ID, "SUB_AGENT_COMPLETED", "COMPLETED", task.OrganizationID)
}

func (s *DefaultSubAgentSpawner) emitEvent(taskID, action, status, orgID string) {
	if s.hub != nil {
		payload := map[string]interface{}{
			"task_id": taskID,
			"action":  action,
			"status":  status,
			// Aesthetic Excellence payload properties
			"ui_blur":     "20px",
			"ui_bg":       "rgba(255, 255, 255, 0.03)",
			"ui_font":     "Outfit, Inter, sans-serif",
			"ui_saturate": "200%",
		}
		s.hub.PublishTaskBroadcast(taskID, payload)
	}
}

func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// Periodic check for orphaned tasks or health metrics.
	// We'll simulate this with a simple loop.
	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return nil
		case <-ticker.C:
			// In a real implementation, we would check for stalled sub-agents here.
		}
	}
}
