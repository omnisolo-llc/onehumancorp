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
	mesh := NewIPCMesh()
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
	mesh := NewIPCMesh()
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
	mesh := NewIPCMesh()
	ctx := context.Background()

	err := mesh.RegisterPresence(ctx, "agent-1", "active")
	if err != nil {
		t.Fatalf("Failed to register presence: %v", err)
	}

	err = mesh.RegisterPresence(ctx, "agent-2", "idle")
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
}

func TestRedisMesh_PubSub(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	mesh := NewRedisMesh(client)
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

	err = mesh.Publish(ctx, "test-topic", []byte("hello-redis"))
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	wg.Wait()
	received := <-msgReceived
	if received != "hello-redis" {
		t.Errorf("Expected 'hello-redis', got '%s'", received)
	}

	err = sub.Close()
	if err != nil {
		t.Fatalf("Failed to close subscription: %v", err)
	}
}

func TestRedisMesh_Locks(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	mesh := NewRedisMesh(client)
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

func TestRedisMesh_Presence(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	mesh := NewRedisMesh(client)
	ctx := context.Background()

	err = mesh.RegisterPresence(ctx, "agent-1", "active")
	if err != nil {
		t.Fatalf("Failed to register presence: %v", err)
	}

	err = mesh.RegisterPresence(ctx, "agent-2", "idle")
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
}
