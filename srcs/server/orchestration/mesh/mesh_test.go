package mesh

import (
	"context"
	"sync"
	"testing"
	"time"
)

func TestLocalMesh(t *testing.T) {
	ctx := context.Background()
	m := NewLocalMesh()

	// Test Pub/Sub
	var wg sync.WaitGroup
	wg.Add(1)
	var received string

	sub, err := m.Subscribe(ctx, "test-topic", func(msg []byte) {
		received = string(msg)
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	m.Publish(ctx, "test-topic", []byte("hello"))
	wg.Wait()
	if received != "hello" {
		t.Fatalf("Expected 'hello', got '%s'", received)
	}

	sub.Unsubscribe()

	// Test Lock
	acquired, err := m.AcquireLock(ctx, "test-lock", 1*time.Second)
	if err != nil || !acquired {
		t.Fatalf("Failed to acquire lock")
	}

	acquired, err = m.AcquireLock(ctx, "test-lock", 1*time.Second)
	if err != nil || acquired {
		t.Fatalf("Should not acquire already held lock")
	}

	m.ReleaseLock(ctx, "test-lock")

	// Test Presence
	m.RegisterPresence(ctx, "agent-1", "IDLE")

	active, err := m.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("GetActiveAgents failed: %v", err)
	}
	if len(active) != 1 || active[0].AgentID != "agent-1" || active[0].Status != "IDLE" {
		t.Fatalf("Unexpected active agents: %+v", active)
	}
}
