package mesh

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/redis/go-redis/v9"
)

// Mini-redis or simple mock isn't provided directly, so we write basic functional tests for LocalMesh
// and logic tests. Redis tests can be skipped if a real redis is not available, or mocked if needed.
// For this task, we will test LocalMesh thoroughly and provide structure for RedisMesh.

func TestLocalMesh_PubSub(t *testing.T) {
	mesh := NewLocalMesh()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	var wg sync.WaitGroup
	wg.Add(1)

	msgReceived := make(chan string, 1)

	sub, err := mesh.Subscribe(ctx, "test-topic", func(msg []byte) {
		msgReceived <- string(msg)
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	err = mesh.Publish(ctx, "test-topic", []byte("hello"))
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	wg.Wait()
	received := <-msgReceived
	if received != "hello" {
		t.Errorf("Expected 'hello', got '%s'", received)
	}

	err = sub.Close()
	if err != nil {
		t.Fatalf("Failed to close subscription: %v", err)
	}
}

func TestLocalMesh_Locks(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	acquired, err := mesh.AcquireLock(ctx, "test-lock", 2*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if !acquired {
		t.Errorf("Expected lock to be acquired")
	}

	// Try again
	acquired, err = mesh.AcquireLock(ctx, "test-lock", 2*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if acquired {
		t.Errorf("Expected lock to fail as it is already held")
	}

	// Release
	err = mesh.ReleaseLock(ctx, "test-lock")
	if err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

	// Try again after release
	acquired, err = mesh.AcquireLock(ctx, "test-lock", 2*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if !acquired {
		t.Errorf("Expected lock to be acquired after release")
	}
}

func TestLocalMesh_Presence(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	err := mesh.RegisterPresence(ctx, "agent1", "IDLE")
	if err != nil {
		t.Fatalf("Failed to register presence: %v", err)
	}

	err = mesh.RegisterPresence(ctx, "agent2", "WORKING")
	if err != nil {
		t.Fatalf("Failed to register presence: %v", err)
	}

	agents, err := mesh.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("Failed to get active agents: %v", err)
	}

	if len(agents) != 2 {
		t.Errorf("Expected 2 active agents, got %d", len(agents))
	}

	foundAgent1 := false
	foundAgent2 := false
	for _, agent := range agents {
		if agent.AgentID == "agent1" && agent.Status == "IDLE" {
			foundAgent1 = true
		}
		if agent.AgentID == "agent2" && agent.Status == "WORKING" {
			foundAgent2 = true
		}
	}

	if !foundAgent1 || !foundAgent2 {
		t.Errorf("Did not find expected agents in active agents list")
	}
}

// For Redis, we typically use something like alicebob/miniredis.
// If it's not in deps, we skip real connections.
func TestRedisMesh_PubSub_NoRealRedis(t *testing.T) {
	// Dummy test to ensure struct exists and compiles
	client := redis.NewClient(&redis.Options{Addr: "localhost:12345"}) // Dummy address
	mesh := NewRedisMesh(client)
	if mesh == nil {
		t.Fatal("Expected mesh to be created")
	}
}
