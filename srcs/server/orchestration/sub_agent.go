package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func writeHeartbeatFile(taskID string, content string) error {
	dir := filepath.Join(".agent-task", "status")
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	path := filepath.Join(dir, fmt.Sprintf("%s.yml", taskID))
	return os.WriteFile(path, []byte(content), 0644)
}

// SubAgentSpawner defines the interface for spawning and monitoring sub-agents.
type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *SharedTask) error
	Monitor(ctx context.Context) error
}

// DefaultSubAgentSpawner implements SubAgentSpawner.
type DefaultSubAgentSpawner struct {
	db       db.Provider
	tm       *TaskManager
	hub      *CentrifugeNode // For teammate mesh broadcasts
	sem      chan struct{} // For concurrency limits in standalone mode
	wg       sync.WaitGroup
	ctx      context.Context
	cancel   context.CancelFunc
}

// NewDefaultSubAgentSpawner creates a new DefaultSubAgentSpawner.
func NewDefaultSubAgentSpawner(provider db.Provider, tm *TaskManager, hub *CentrifugeNode, concurrency int) *DefaultSubAgentSpawner {
	ctx, cancel := context.WithCancel(context.Background())
	return &DefaultSubAgentSpawner{
		db:     provider,
		tm:     tm,
		hub:    hub,
		sem:    make(chan struct{}, concurrency),
		ctx:    ctx,
		cancel: cancel,
	}
}

// Spawn spawns a new sub-agent for the given task.
func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *SharedTask) error {
	// Emit SUB_AGENT_SPAWNED event
	if s.hub != nil {
		payload := map[string]interface{}{
			"task_id":  task.ID,
			"action":   "SUB_AGENT_SPAWNED",
			"agent_id": "sub-agent-spawner",
			"status":   "IN_PROGRESS",
		}
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}

	if s.db.IsSQLite() {
		// Standalone mode: spawn local goroutine with concurrency limit
		select {
		case <-s.ctx.Done():
			return s.ctx.Err()
		case <-ctx.Done():
			return ctx.Err()
		case s.sem <- struct{}{}:
			// Acquired semaphore
		}
		s.wg.Add(1)
		go func() {
			defer s.wg.Done()
			defer func() { <-s.sem }()
			s.executeWithRetry(task)
		}()
	} else {
		// Cloud mode: Here we would request a new K8s pod or use a distributed worker.
		// For now, we simulate async execution using a goroutine without the strict local semaphore.
		s.wg.Add(1)
		go func() {
			defer s.wg.Done()
			s.executeWithRetry(task)
		}()
	}

	return nil
}

func (s *DefaultSubAgentSpawner) executeWithRetry(task *SharedTask) {
	maxRetries := 3
	backoff := 100 * time.Millisecond

	for i := 0; i < maxRetries; i++ {
		select {
		case <-s.ctx.Done():
			return
		default:
		}

		err := s.executeTask(task)
		if err == nil {
			_ = s.completeTask(task)
			return
		}

		time.Sleep(backoff)
		backoff *= 2
	}

	_ = s.failTask(task)
}

func (s *DefaultSubAgentSpawner) failTask(task *SharedTask) error {
	_, err := s.db.Exec(context.Background(), "UPDATE shared_tasks SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1", task.ID)
	if err != nil {
		return fmt.Errorf("failed to fail sub-agent task: %w", err)
	}

	// Emit SUB_AGENT_FAILED event
	if s.hub != nil {
		payload := map[string]interface{}{
			"task_id":  task.ID,
			"action":   "SUB_AGENT_FAILED",
			"agent_id": "sub-agent-spawner",
			"status":   "FAILED",
		}
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}

	return nil
}

func (s *DefaultSubAgentSpawner) executeTask(task *SharedTask) error {
	// Extract payload configuration
	var subAgentType string
	var parentTaskID string
	var isolatedContext bool

	if len(task.Payload) > 0 {
		var payload map[string]interface{}
		if err := json.Unmarshal(task.Payload, &payload); err == nil {
			if v, ok := payload["sub_agent_type"].(string); ok {
				subAgentType = v
			}
			if v, ok := payload["parent_task_id"].(string); ok {
				parentTaskID = v
			}
			if v, ok := payload["isolated_context"].(bool); ok {
				isolatedContext = v
			}
		}
	}

	// Just checking these values to silence compiler and log intention
	_ = subAgentType
	_ = parentTaskID
	_ = isolatedContext

	// Emit heartbeat to .agent-task/status/{taskID}.yml
	heartbeatContent := fmt.Sprintf(`---
task_id: %s
status: IN_PROGRESS
timestamp: %d
sub_agent_type: %s
parent_task_id: %s
isolated_context: %t
---`, task.ID, time.Now().Unix(), subAgentType, parentTaskID, isolatedContext)

	_ = writeHeartbeatFile(task.ID, heartbeatContent)

	// Simulate real work that might fail
	select {
	case <-s.ctx.Done():
		return s.ctx.Err()
	case <-time.After(100 * time.Millisecond):
	}
	return nil
}

func (s *DefaultSubAgentSpawner) completeTask(task *SharedTask) error {
	// Create an admin context for task completion since this is a system worker
	// Use an explicit background context with necessary claims if needed
	// For simplicity in this background worker, we might just use the raw DB query

	// Mark task completed
	_, err := s.db.Exec(context.Background(), "UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1", task.ID)
	if err != nil {
		return fmt.Errorf("failed to complete sub-agent task: %w", err)
	}

	// Emit SUB_AGENT_COMPLETED event
	if s.hub != nil {
		payload := map[string]interface{}{
			"task_id":  task.ID,
			"action":   "SUB_AGENT_COMPLETED",
			"agent_id": "sub-agent-spawner",
			"status":   "COMPLETED",
		}
		s.hub.PublishTaskBroadcast(task.ID, payload)
	}

	return nil
}

// Monitor is a loop that could be used for heartbeats or checking sub-agent health.
func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-s.ctx.Done():
			return nil
		case <-ctx.Done():
			return nil
		case <-ticker.C:
			// Implement heartbeat/monitoring logic here if needed
		}
	}
}

// Stop gracefully shuts down the spawner.
func (s *DefaultSubAgentSpawner) Stop() {
	s.cancel()
	s.wg.Wait()
}
