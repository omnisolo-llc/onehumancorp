package mesh

import (
	"context"
	"testing"
	"time"
)

func TestLocalMesh(t *testing.T) {
	lm := NewLocalMesh()
	ctx := context.Background()

	// Test Pub/Sub
	received := make(chan []byte, 1)
	sub, err := lm.Subscribe(ctx, "test_topic", func(msg []byte) {
		received <- msg
	})
	if err != nil {
		t.Fatalf("Subscribe error: %v", err)
	}

	err = lm.Publish(ctx, "test_topic", []byte("hello"))
	if err != nil {
		t.Fatalf("Publish error: %v", err)
	}

	select {
	case msg := <-received:
		if string(msg) != "hello" {
			t.Errorf("Expected 'hello', got %s", msg)
		}
	case <-time.After(time.Second):
		t.Errorf("Publish/Subscribe timed out")
	}

	sub.Unsubscribe()

	// Test Lock
	locked, err := lm.AcquireLock(ctx, "my_lock", "token1", time.Second*5)
	if err != nil || !locked {
		t.Errorf("Expected to acquire lock, locked=%v, err=%v", locked, err)
	}

	locked2, err := lm.AcquireLock(ctx, "my_lock", "token2", time.Second*5)
	if err != nil || locked2 {
		t.Errorf("Expected NOT to acquire lock, locked=%v, err=%v", locked2, err)
	}

	err = lm.ReleaseLock(ctx, "my_lock", "wrong_token")
	if err != nil {
		t.Errorf("ReleaseLock error: %v", err)
	}

	// Lock should still be held
	locked3, err := lm.AcquireLock(ctx, "my_lock", "token3", time.Second*5)
	if err != nil || locked3 {
		t.Errorf("Expected NOT to acquire lock, locked=%v, err=%v", locked3, err)
	}

	err = lm.ReleaseLock(ctx, "my_lock", "token1")
	if err != nil {
		t.Errorf("ReleaseLock error: %v", err)
	}

	locked4, err := lm.AcquireLock(ctx, "my_lock", "token4", time.Second*5)
	if err != nil || !locked4 {
		t.Errorf("Expected to acquire lock again, locked=%v, err=%v", locked4, err)
	}

	// Test Presence
	err = lm.RegisterPresence(ctx, "agent1", "IDLE")
	if err != nil {
		t.Errorf("RegisterPresence error: %v", err)
	}

	agents, err := lm.GetActiveAgents(ctx)
	if err != nil {
		t.Errorf("GetActiveAgents error: %v", err)
	}
	if len(agents) != 1 || agents[0].AgentID != "agent1" || agents[0].Status != "IDLE" {
		t.Errorf("Unexpected active agents: %+v", agents)
	}
}
