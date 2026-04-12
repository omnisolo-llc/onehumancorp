package mesh

import (
	"context"
	"sync"
	"testing"
	"time"
)

func TestLocalMesh(t *testing.T) {
	m := NewLocalMesh()
	ctx := context.Background()

	// Test PubSub
	var wg sync.WaitGroup
	wg.Add(1)
	var received string
	sub, err := m.Subscribe(ctx, "test_topic", func(msg []byte) {
		received = string(msg)
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Subscribe error: %v", err)
	}

	err = m.Publish(ctx, "test_topic", []byte("hello world"))
	if err != nil {
		t.Fatalf("Publish error: %v", err)
	}
	wg.Wait()
	if received != "hello world" {
		t.Errorf("expected 'hello world', got '%s'", received)
	}
	sub.Unsubscribe()

	// Test Locks
	ok, err := m.AcquireLock(ctx, "lock1", 1*time.Second)
	if err != nil || !ok {
		t.Errorf("failed to acquire lock")
	}
	ok, err = m.AcquireLock(ctx, "lock1", 1*time.Second)
	if err != nil || ok {
		t.Errorf("should not be able to acquire lock again")
	}
	m.ReleaseLock(ctx, "lock1")
	ok, err = m.AcquireLock(ctx, "lock1", 1*time.Second)
	if err != nil || !ok {
		t.Errorf("failed to acquire lock after release")
	}

	// Test Presence
	err = m.RegisterPresence(ctx, "agent1", "WORKING")
	if err != nil {
		t.Fatalf("RegisterPresence error: %v", err)
	}
	agents, err := m.GetActiveAgents(ctx)
	if err != nil || len(agents) != 1 || agents[0].AgentID != "agent1" || agents[0].Status != "WORKING" {
		t.Errorf("GetActiveAgents failed: %v", agents)
	}
}
