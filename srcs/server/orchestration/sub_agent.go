package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"math/rand"
	"os"
	"path/filepath"
	"time"
)

// SubAgentSpawner defines the interface for spawning and monitoring sub-agents.
type SubAgentSpawner interface {
	Spawn(ctx context.Context, task *SharedTask) error
	Monitor(ctx context.Context) error
}

type DefaultSubAgentSpawner struct {
	mesh      MeshHub
	isSQLite  bool
	semaphore chan struct{}
}

// NewDefaultSubAgentSpawner creates a new instance of DefaultSubAgentSpawner.
func NewDefaultSubAgentSpawner(mesh MeshHub, isSQLite bool, maxConcurrency int) *DefaultSubAgentSpawner {
	var sem chan struct{}
	if isSQLite {
		if maxConcurrency <= 0 {
			maxConcurrency = 5 // default fallback limit
		}
		sem = make(chan struct{}, maxConcurrency)
	}
	return &DefaultSubAgentSpawner{
		mesh:      mesh,
		isSQLite:  isSQLite,
		semaphore: sem,
	}
}

// Spawn triggers the creation of a sub-agent for the delegated task.
func (s *DefaultSubAgentSpawner) Spawn(ctx context.Context, task *SharedTask) error {
	// Broadcast SUB_AGENT_SPAWNED
	s.broadcastLifecycleEvent(ctx, task.ID, "SUB_AGENT_SPAWNED")

	if s.isSQLite {
		// Throttled spawning for Standalone mode
		s.semaphore <- struct{}{}
		go func(t *SharedTask) {
			defer func() { <-s.semaphore }()
			s.runSubAgent(context.Background(), t)
		}(task)
		return nil
	}

	// Unthrottled distributed spawning for Cloud mode
	go s.runSubAgent(context.Background(), task)
	return nil
}

// runSubAgent simulates the sub-agent execution with retries and heartbeats.
func (s *DefaultSubAgentSpawner) runSubAgent(ctx context.Context, task *SharedTask) {
	maxRetries := 3
	backoff := time.Second

	for attempt := 1; attempt <= maxRetries; attempt++ {
		err := s.executeTask(ctx, task)
		if err == nil {
			s.broadcastLifecycleEvent(ctx, task.ID, "SUB_AGENT_COMPLETED")
			return
		}

		if attempt == maxRetries {
			s.broadcastLifecycleEvent(ctx, task.ID, "SUB_AGENT_FAILED")
			return
		}

		// Exponential backoff
		time.Sleep(backoff)
		backoff *= 2
	}
}

func (s *DefaultSubAgentSpawner) executeTask(ctx context.Context, task *SharedTask) error {
	// Write heartbeat to .agent-task/status/
	statusDir := filepath.Join(".agent-task", "status")
	if err := os.MkdirAll(statusDir, 0755); err != nil {
		return err
	}

	statusFile := filepath.Join(statusDir, fmt.Sprintf("%s.json", task.ID))

	// Simulated heartbeat loop and potential transient failure
	for i := 0; i < 3; i++ {
		statusData := map[string]interface{}{
			"task_id":   task.ID,
			"status":    "RUNNING",
			"timestamp": time.Now().Unix(),
			"progress":  fmt.Sprintf("%d/3", i+1),
		}
		statusBytes, _ := json.Marshal(statusData)
		_ = os.WriteFile(statusFile, statusBytes, 0644)

		// Simulate transient failure randomly (10% chance)
		if rand.Float32() < 0.10 {
			return fmt.Errorf("transient sub-agent failure")
		}

		time.Sleep(10 * time.Millisecond)
	}

	// Mark final completion status
	finalData := map[string]interface{}{
		"task_id":   task.ID,
		"status":    "COMPLETED",
		"timestamp": time.Now().Unix(),
	}
	finalBytes, _ := json.Marshal(finalData)
	_ = os.WriteFile(statusFile, finalBytes, 0644)

	return nil
}

func (s *DefaultSubAgentSpawner) broadcastLifecycleEvent(ctx context.Context, taskID string, eventType string) {
	if s.mesh == nil {
		return
	}

	payload := map[string]interface{}{
		"task_id": taskID,
		"event":   eventType,
		"time":    time.Now().Unix(),
	}
	bytes, _ := json.Marshal(payload)
	// The problem description mentioned "mesh:tasks" channels
	_ = s.mesh.Publish(ctx, "mesh:tasks", bytes)
}

func (s *DefaultSubAgentSpawner) Monitor(ctx context.Context) error {
	// In a real system, Monitor might clean up stale lock files or failed pods.
	// We'll leave it as a simple placeholder loop.
	return nil
}
