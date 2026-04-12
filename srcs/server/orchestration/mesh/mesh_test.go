package mesh

import (
	"context"
	"testing"
	"time"
)

func TestLocalMesh_PubSub(t *testing.T) {
	lm := NewLocalTeammateMesh()
	ctx := context.Background()

	topic := "test-topic"
	msgCh := make(chan []byte, 1)

	sub, err := lm.Subscribe(ctx, topic, func(msg []byte) {
		msgCh <- msg
	})
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}
	defer sub.Close()

	payload := []byte("hello")
	if err := lm.Publish(ctx, topic, payload); err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	select {
	case msg := <-msgCh:
		if string(msg) != string(payload) {
			t.Errorf("Expected %s, got %s", payload, msg)
		}
	case <-time.After(1 * time.Second):
		t.Errorf("Timeout waiting for message")
	}
}

func TestLocalMesh_Locking(t *testing.T) {
	lm := NewLocalTeammateMesh()
	ctx := context.Background()
	key := "test-lock"

	acquired, err := lm.AcquireLock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if !acquired {
		t.Errorf("Expected to acquire lock")
	}

	acquired2, err := lm.AcquireLock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock 2: %v", err)
	}
	if acquired2 {
		t.Errorf("Expected to fail acquiring lock again")
	}

	if err := lm.ReleaseLock(ctx, key); err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

	acquired3, err := lm.AcquireLock(ctx, key, 1*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock 3: %v", err)
	}
	if !acquired3 {
		t.Errorf("Expected to acquire lock after release")
	}
}

func TestLocalMesh_Presence(t *testing.T) {
	lm := NewLocalTeammateMesh()
	ctx := context.Background()

	err := lm.RegisterPresence(ctx, "agent1", "IDLE")
	if err != nil {
		t.Fatalf("Failed to register presence: %v", err)
	}

	agents, err := lm.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("Failed to get active agents: %v", err)
	}

	if len(agents) != 1 {
		t.Errorf("Expected 1 active agent, got %d", len(agents))
	} else if agents[0].AgentID != "agent1" || agents[0].Status != "IDLE" {
		t.Errorf("Expected agent1 IDLE, got %v", agents[0])
	}
}
