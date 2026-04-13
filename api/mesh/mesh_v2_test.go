package mesh

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/redis/rueidis"
)

func TestLocalMeshV2(t *testing.T) {
	lm := NewLocalMeshV2()
	testMeshV2(t, lm)
}

func TestRedisMeshV2(t *testing.T) {
	opt, err := rueidis.ParseURL("redis://localhost:6379")
	if err != nil {
		t.Skip("Redis is not running, skipping RedisMeshV2 tests")
	}
	client, err := rueidis.NewClient(opt)
	if err != nil {
		t.Skip("Redis is not running, skipping RedisMeshV2 tests")
	}

	ctx := context.Background()

	client.Do(ctx, client.B().Flushdb().Build())
	defer client.Close()

	rm := NewRedisMeshV2(client)
	testMeshV2(t, rm)
}

func testMeshV2(t *testing.T, mesh TeammateMeshV2) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})

	// Test PubSub
	ch := make(chan string, 1)
	sub, err := mesh.Subscribe(ctx, "test-topic", func(msg []byte) {
		ch <- string(msg)
	})
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	err = mesh.Publish(ctx, "test-topic", []byte("hello"))
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	select {
	case msg := <-ch:
		if msg != "hello" {
			t.Errorf("Expected 'hello', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}

	sub.Unsubscribe()

	// Test Distributed Lock
	ok, err := mesh.AcquireLock(ctx, "test-lock", "lock-1", 1*time.Second)
	if err != nil || !ok {
		t.Fatalf("Failed to acquire lock: %v, ok: %v", err, ok)
	}

	ok, err = mesh.AcquireLock(ctx, "test-lock", "lock-2", 1*time.Second)
	if err != nil || ok {
		t.Fatalf("Should not acquire lock: %v, ok: %v", err, ok)
	}

	err = mesh.ReleaseLock(ctx, "test-lock", "lock-1")
	if err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

	ok, err = mesh.AcquireLock(ctx, "test-lock", "lock-2", 1*time.Second)
	if err != nil || !ok {
		t.Fatalf("Failed to acquire lock after release: %v, ok: %v", err, ok)
	}

	// Test Presence
	err = mesh.RegisterPresence(ctx, "agent-1", "IDLE")
	if err != nil {
		t.Fatalf("Failed to register presence: %v", err)
	}

	err = mesh.RegisterPresence(ctx, "agent-1", "WORKING")
	if err != nil {
		t.Fatalf("Failed to register presence update: %v", err)
	}

	agents, err := mesh.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("Failed to get active agents: %v", err)
	}

	if len(agents) != 1 || agents[0].AgentID != "agent-1" || agents[0].Status != "WORKING" {
		t.Fatalf("Unexpected active agents: %+v", agents)
	}
}

func TestAuthErrorsV2(t *testing.T) {
	ctx := context.Background() // No claims
	svc := NewLocalMeshV2()

	err := svc.Publish(ctx, "topic", []byte("hello"))
	if err == nil {
		t.Error("expected unauthorized error")
	}

	_, err = svc.Subscribe(ctx, "topic", func(msg []byte) {})
	if err == nil {
		t.Error("expected unauthorized error")
	}
}
