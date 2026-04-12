package orchestration

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestLocalTeammateMeshPubSub(t *testing.T) {
	// Initialize a dummy db provider for testing if possible,
	// or just pass nil since we only test pubsub
	lm := NewLocalTeammateMesh(&db.TestProvider{})

	var wg sync.WaitGroup
	var received int
	var mu sync.Mutex

	ctx := context.Background()

	// Subscribe
	sub, err := lm.Subscribe(ctx, "test_topic", func(msg []byte) {
		mu.Lock()
		received++
		mu.Unlock()
		wg.Done()
	})

	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	// Publish
	wg.Add(1)
	err = lm.Publish(ctx, "test_topic", []byte("hello"))
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	wg.Wait()

	mu.Lock()
	if received != 1 {
		t.Errorf("Expected 1 message, got %d", received)
	}
	mu.Unlock()

	// Test Unsubscribe
	err = sub.Close()
	if err != nil {
		t.Fatalf("Failed to close subscription: %v", err)
	}

	err = lm.Publish(ctx, "test_topic", []byte("hello again"))
	if err != nil {
		t.Fatalf("Failed to publish after close: %v", err)
	}

	// Give a little time to ensure no message is received
	time.Sleep(50 * time.Millisecond)

	mu.Lock()
	if received != 1 {
		t.Errorf("Expected 1 message after unsubscribe, got %d", received)
	}
	mu.Unlock()
}

func TestLocalTeammateMeshPresence(t *testing.T) {
	lm := NewLocalTeammateMesh(&db.TestProvider{})
	ctx := context.Background()

	err := lm.RegisterPresence(ctx, "agent1", "WORKING")
	if err != nil {
		t.Fatalf("Failed to register presence: %v", err)
	}

	agents, err := lm.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("Failed to get active agents: %v", err)
	}

	if len(agents) != 1 {
		t.Errorf("Expected 1 active agent, got %d", len(agents))
	}

	if agents[0].AgentID != "agent1" || agents[0].Status != "WORKING" {
		t.Errorf("Unexpected agent details: %+v", agents[0])
	}
}
