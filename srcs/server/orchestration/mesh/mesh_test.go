package mesh

import (
	"context"
	"sync"
	"testing"
	"time"
)

func TestLocalTeammateMesh_PublishSubscribe(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx := context.Background()

	var wg sync.WaitGroup
	wg.Add(1)

	var received string

	sub, err := mesh.Subscribe(ctx, "test_topic", func(msg []byte) {
		received = string(msg)
		wg.Done()
	})

	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if sub == nil {
		t.Fatalf("expected subscription, got nil")
	}

	err = mesh.Publish(ctx, "test_topic", []byte("hello world"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	wg.Wait()

	if received != "hello world" {
		t.Fatalf("expected 'hello world', got %v", received)
	}

	err = sub.Unsubscribe(ctx)
	if err != nil {
		t.Fatalf("expected no error on unsubscribe, got %v", err)
	}
}

func TestLocalTeammateMesh_Locking(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx := context.Background()

	acquired, err := mesh.AcquireLock(ctx, "my_lock", 100*time.Millisecond)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if !acquired {
		t.Fatalf("expected to acquire lock")
	}

	// Try acquiring again, should fail
	acquired, err = mesh.AcquireLock(ctx, "my_lock", 100*time.Millisecond)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if acquired {
		t.Fatalf("expected NOT to acquire lock again")
	}

	// Release lock
	err = mesh.ReleaseLock(ctx, "my_lock")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Try acquiring again after release, should succeed
	acquired, err = mesh.AcquireLock(ctx, "my_lock", 100*time.Millisecond)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if !acquired {
		t.Fatalf("expected to acquire lock after release")
	}
}

func TestLocalTeammateMesh_Presence(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx := context.Background()

	err := mesh.RegisterPresence(ctx, "agent1", "IDLE")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = mesh.RegisterPresence(ctx, "agent2", "WORKING")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	agents, err := mesh.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(agents) != 2 {
		t.Fatalf("expected 2 agents, got %d", len(agents))
	}

	foundAgent1 := false
	for _, a := range agents {
		if a.AgentID == "agent1" && a.Status == "IDLE" {
			foundAgent1 = true
		}
	}

	if !foundAgent1 {
		t.Fatalf("expected to find agent1 with IDLE status")
	}
}
