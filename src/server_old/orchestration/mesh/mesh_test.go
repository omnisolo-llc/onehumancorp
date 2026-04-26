package mesh

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

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

	token, acquired, err := mesh.AcquireLock(ctx, "test-lock", 2*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if !acquired {
		t.Errorf("Expected lock to be acquired")
	}

	// Try again
	_, acquired, err = mesh.AcquireLock(ctx, "test-lock", 2*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if acquired {
		t.Errorf("Expected lock to fail as it is already held")
	}

	// Try to release with wrong token
	err = mesh.ReleaseLock(ctx, "test-lock", "wrong-token")
	if err == nil {
		t.Fatalf("Expected error when releasing with wrong token")
	}
	// Verify lock is still held
	_, acquired, err = mesh.AcquireLock(ctx, "test-lock", 2*time.Second)
	if acquired {
		t.Errorf("Expected lock to fail as wrong token release should have failed")
	}

	// Release
	err = mesh.ReleaseLock(ctx, "test-lock", token)
	if err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

	// Try again after release
	_, acquired, err = mesh.AcquireLock(ctx, "test-lock", 2*time.Second)
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

func setupRedis(t *testing.T) (*miniredis.Miniredis, *RedisMesh) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}

	client := redis.NewClient(&redis.Options{
		Addr: s.Addr(),
	})

	mesh := NewRedisMesh(client)
	return s, mesh
}

func TestRedisMesh_PubSub(t *testing.T) {
	mr, mesh := setupRedis(t)
	defer mr.Close()

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

func TestRedisMesh_Locks(t *testing.T) {
	mr, mesh := setupRedis(t)
	defer mr.Close()

	ctx := context.Background()

	token, acquired, err := mesh.AcquireLock(ctx, "test-lock", 10*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if !acquired {
		t.Errorf("Expected lock to be acquired")
	}

	// Try again
	_, acquired, err = mesh.AcquireLock(ctx, "test-lock", 10*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if acquired {
		t.Errorf("Expected lock to fail as it is already held")
	}

	// Try release wrong token
	err = mesh.ReleaseLock(ctx, "test-lock", "wrong-token")
	if err == nil {
		t.Fatalf("Expected error when releasing with wrong token")
	}

	// Release
	err = mesh.ReleaseLock(ctx, "test-lock", token)
	if err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

	// Try again after release
	_, acquired, err = mesh.AcquireLock(ctx, "test-lock", 10*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if !acquired {
		t.Errorf("Expected lock to be acquired after release")
	}
}

func TestRedisMesh_Presence(t *testing.T) {
	mr, mesh := setupRedis(t)
	defer mr.Close()

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
