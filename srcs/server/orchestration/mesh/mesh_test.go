package mesh

import (
	"context"
	"sync"
	"testing"
	"time"
)

func TestLocalMesh(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	// Test Pub/Sub
	var wg sync.WaitGroup
	wg.Add(1)
	sub, err := mesh.Subscribe(ctx, "test_topic", func(msg []byte) {
		if string(msg) != "hello" {
			t.Errorf("Expected 'hello', got '%s'", string(msg))
		}
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	err = mesh.Publish(ctx, "test_topic", []byte("hello"))
	if err != nil {
		t.Fatalf("Publish failed: %v", err)
	}

	wg.Wait()
	sub.Close()

	// Test Distributed Lock
	acquired, err := mesh.AcquireLock(ctx, "my_lock", 1*time.Second)
	if err != nil || !acquired {
		t.Fatalf("Failed to acquire lock: %v", err)
	}

	acquired, err = mesh.AcquireLock(ctx, "my_lock", 1*time.Second)
	if err != nil || acquired {
		t.Fatalf("Acquired lock that was already held")
	}

	err = mesh.ReleaseLock(ctx, "my_lock")
	if err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

	acquired, err = mesh.AcquireLock(ctx, "my_lock", 1*time.Second)
	if err != nil || !acquired {
		t.Fatalf("Failed to acquire lock after release: %v", err)
	}

	// Test Presence
	err = mesh.RegisterPresence(ctx, "agent1", "IDLE")
	if err != nil {
		t.Fatalf("RegisterPresence failed: %v", err)
	}

	agents, err := mesh.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("GetActiveAgents failed: %v", err)
	}
	if len(agents) != 1 || agents[0].AgentID != "agent1" || agents[0].Status != "IDLE" {
		t.Fatalf("Unexpected agents list: %+v", agents)
	}
}
