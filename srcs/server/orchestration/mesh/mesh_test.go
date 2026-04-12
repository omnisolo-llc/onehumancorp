package mesh

import (
	"context"
	"testing"
	"time"
)

func TestLocalMesh_PubSub(t *testing.T) {
	lm := NewLocalMesh()
	ctx := context.Background()

	received := make(chan []byte, 1)
	sub, err := lm.Subscribe(ctx, "test_topic", func(msg []byte) {
		received <- msg
	})
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}
	defer sub.Close()

	if err := lm.Publish(ctx, "test_topic", []byte("hello")); err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	select {
	case msg := <-received:
		if string(msg) != "hello" {
			t.Errorf("Expected 'hello', got '%s'", string(msg))
		}
	case <-time.After(1 * time.Second):
		t.Errorf("Timeout waiting for message")
	}
}

func TestLocalMesh_DistributedLock(t *testing.T) {
	lm := NewLocalMesh()
	ctx := context.Background()

	acquired, err := lm.AcquireLock(ctx, "task1", 1*time.Second)
	if err != nil || !acquired {
		t.Fatalf("Failed to acquire lock: %v", err)
	}

	acquired2, err := lm.AcquireLock(ctx, "task1", 1*time.Second)
	if err != nil || acquired2 {
		t.Fatalf("Should not acquire already held lock")
	}

	lm.ReleaseLock(ctx, "task1")

	acquired3, err := lm.AcquireLock(ctx, "task1", 1*time.Second)
	if err != nil || !acquired3 {
		t.Fatalf("Failed to acquire lock after release: %v", err)
	}
}

func TestLocalMesh_Presence(t *testing.T) {
	lm := NewLocalMesh()
	ctx := context.Background()

	err := lm.RegisterPresence(ctx, "agent1", "IDLE")
	if err != nil {
		t.Fatalf("Failed to register presence: %v", err)
	}

	agents, err := lm.GetActiveAgents(ctx)
	if err != nil || len(agents) != 1 || agents[0].AgentID != "agent1" || agents[0].Status != "IDLE" {
		t.Fatalf("Failed to get active agents: %v", agents)
	}
}

func TestRedisMesh_Initialization(t *testing.T) {
	_, err := NewRedisMesh("invalid-url")
	if err == nil {
		t.Fatalf("Expected error with invalid redis URL")
	}
}
