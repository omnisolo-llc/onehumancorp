package mesh

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestRedisMesh(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to run miniredis: %v", err)
	}
	defer s.Close()

	client := redis.NewClient(&redis.Options{
		Addr: s.Addr(),
	})
	defer client.Close()

	mesh := NewRedisMesh(client)
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

	// Wait for subscription to establish
	time.Sleep(100 * time.Millisecond)

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
