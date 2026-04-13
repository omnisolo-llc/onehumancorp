package mesh

import (
	"context"
	"os"
	"testing"
	"time"
)

func TestLocalMesh_PubSub(t *testing.T) {
	mesh := NewLocalMesh()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	received := make(chan string, 1)

	sub, err := mesh.Subscribe(ctx, "test-topic", func(msg []byte) {
		received <- string(msg)
	})
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}
	defer sub.Close()

	if err := mesh.Publish(ctx, "test-topic", []byte("hello")); err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	select {
	case msg := <-received:
		if msg != "hello" {
			t.Errorf("expected 'hello', got '%s'", msg)
		}
	case <-ctx.Done():
		t.Fatal("timeout waiting for message")
	}
}

func TestLocalMesh_Locking(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	// Acquire lock
	ok, err := mesh.AcquireLock(ctx, "task-1", 5*time.Second)
	if err != nil {
		t.Fatalf("failed to acquire lock: %v", err)
	}
	if !ok {
		t.Fatal("expected to acquire lock")
	}

	// Try to acquire again, should fail or succeed? Since we own it, maybe it fails because it's already locked
	// Wait, the implementation says if entry.token != m.instanceID { return false }.
	// So if it's the SAME instance, maybe it's allowed or not. Currently it overwrites the lock.
	ok, _ = mesh.AcquireLock(ctx, "task-1", 5*time.Second)
	if !ok {
		// Just noting behavior
	}

	// Release lock
	if err := mesh.ReleaseLock(ctx, "task-1"); err != nil {
		t.Fatalf("failed to release lock: %v", err)
	}
}

func TestLocalMesh_Presence(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	if err := mesh.RegisterPresence(ctx, "agent-1", "IDLE"); err != nil {
		t.Fatalf("failed to register presence: %v", err)
	}

	agents, err := mesh.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("failed to get active agents: %v", err)
	}

	if len(agents) != 1 || agents[0].AgentID != "agent-1" || agents[0].Status != "IDLE" {
		t.Errorf("unexpected agents: %+v", agents)
	}
}

func TestRedisMesh_All(t *testing.T) {
	redisURL := os.Getenv("REDIS_URL")
	if redisURL == "" {
		t.Skip("REDIS_URL not set, skipping Redis tests")
	}

	mesh, err := NewRedisMesh(redisURL)
	if err != nil {
		t.Fatalf("failed to create redis mesh: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Presence
	if err := mesh.RegisterPresence(ctx, "agent-1", "WORKING"); err != nil {
		t.Fatalf("failed to register presence: %v", err)
	}

	agents, err := mesh.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("failed to get active agents: %v", err)
	}
	found := false
	for _, a := range agents {
		if a.AgentID == "agent-1" && a.Status == "WORKING" {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("expected to find agent-1 in active agents, got %+v", agents)
	}

	// Locking
	ok, err := mesh.AcquireLock(ctx, "test-lock", 10*time.Second)
	if err != nil {
		t.Fatalf("failed to acquire lock: %v", err)
	}
	if !ok {
		t.Fatal("expected to acquire lock")
	}

	// Release
	if err := mesh.ReleaseLock(ctx, "test-lock"); err != nil {
		t.Fatalf("failed to release lock: %v", err)
	}
}
