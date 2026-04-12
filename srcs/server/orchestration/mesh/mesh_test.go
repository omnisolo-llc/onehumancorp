package mesh

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
)

func TestLocalMesh_PubSub(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	var wg sync.WaitGroup
	wg.Add(1)

	sub, err := mesh.Subscribe(ctx, "test-topic", func(msg []byte) {
		if string(msg) != "hello" {
			t.Errorf("expected hello, got %s", string(msg))
		}
		wg.Done()
	})
	if err != nil {
		t.Fatalf("subscribe failed: %v", err)
	}

	err = mesh.Publish(ctx, "test-topic", []byte("hello"))
	if err != nil {
		t.Fatalf("publish failed: %v", err)
	}

	wg.Wait()
	err = sub.Unsubscribe()
	if err != nil {
		t.Fatalf("unsubscribe failed: %v", err)
	}
}

func TestLocalMesh_Lock(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	ok, err := mesh.AcquireLock(ctx, "my-lock", 1*time.Second, "my-token")
	if err != nil || !ok {
		t.Fatalf("expected to acquire lock")
	}

	ok, err = mesh.AcquireLock(ctx, "my-lock", 1*time.Second, "my-token")
	if err != nil || ok {
		t.Fatalf("expected to fail acquiring lock")
	}

	err = mesh.ReleaseLock(ctx, "my-lock", "my-token")
	if err != nil {
		t.Fatalf("release failed: %v", err)
	}

	ok, err = mesh.AcquireLock(ctx, "my-lock", 1*time.Second, "my-token")
	if err != nil || !ok {
		t.Fatalf("expected to acquire lock again")
	}
}

func TestLocalMesh_Presence(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	err := mesh.RegisterPresence(ctx, "agent1", "IDLE")
	if err != nil {
		t.Fatalf("register failed: %v", err)
	}

	agents, err := mesh.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("get agents failed: %v", err)
	}
	if len(agents) != 1 || agents[0].AgentID != "agent1" || agents[0].Status != "IDLE" {
		t.Fatalf("unexpected agents: %v", agents)
	}
}

func TestRedisMesh_All(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("miniredis failed: %v", err)
	}
	defer mr.Close()

	mesh, err := NewRedisMesh("redis://" + mr.Addr())
	if err != nil {
		t.Fatalf("NewRedisMesh failed: %v", err)
	}
	ctx := context.Background()

	// 1. PubSub
	var wg sync.WaitGroup
	wg.Add(1)

	sub, err := mesh.Subscribe(ctx, "test-topic", func(msg []byte) {
		if string(msg) != "hello" {
			t.Errorf("expected hello, got %s", string(msg))
		}
		wg.Done()
	})
	if err != nil {
		t.Fatalf("subscribe failed: %v", err)
	}

	// wait for subscribe to be ready
	time.Sleep(100 * time.Millisecond)

	err = mesh.Publish(ctx, "test-topic", []byte("hello"))
	if err != nil {
		t.Fatalf("publish failed: %v", err)
	}

	wg.Wait()
	sub.Unsubscribe()

	// 2. Lock
	ok, err := mesh.AcquireLock(ctx, "my-lock", 1*time.Second, "my-token")
	if err != nil || !ok {
		t.Fatalf("expected to acquire lock")
	}

	ok, err = mesh.AcquireLock(ctx, "my-lock", 1*time.Second, "my-token")
	if err != nil || ok {
		t.Fatalf("expected to fail acquiring lock")
	}

	err = mesh.ReleaseLock(ctx, "my-lock", "my-token")
	if err != nil {
		t.Fatalf("release failed: %v", err)
	}

	// 3. Presence
	err = mesh.RegisterPresence(ctx, "agent1", "IDLE")
	if err != nil {
		t.Fatalf("register failed: %v", err)
	}

	agents, err := mesh.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("get agents failed: %v", err)
	}
	if len(agents) != 1 || agents[0].AgentID != "agent1" || agents[0].Status != "IDLE" {
		t.Fatalf("unexpected agents: %v", agents)
	}
}
