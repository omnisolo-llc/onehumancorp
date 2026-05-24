package kairos

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func setupTestMesh(t *testing.T) (*miniredis.Miniredis, TeammateMesh, *redis.Client) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}

	client := redis.NewClient(&redis.Options{
		Addr: s.Addr(),
	})

	mesh := NewTeammateMesh(client)
	return s, mesh, client
}

func TestTeammateMesh_PublishAndSubscribeTaskCreated(t *testing.T) {
	s, mesh, client := setupTestMesh(t)
	defer s.Close()
	defer client.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	ch := mesh.SubscribeTaskCreated(ctx)

	// Ensure subscription is established before publishing
	time.Sleep(100 * time.Millisecond)

	err := mesh.PublishTaskCreated(ctx, "mission-123")
	if err != nil {
		t.Fatalf("unexpected error publishing task created: %v", err)
	}

	select {
	case msg := <-ch:
		if msg != "mission-123" {
			t.Errorf("expected 'mission-123', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Errorf("timeout waiting for message")
	}
}

func TestTeammateMesh_PublishAndSubscribeStatusUpdate(t *testing.T) {
	s, mesh, client := setupTestMesh(t)
	defer s.Close()
	defer client.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	ch := mesh.SubscribeStatusUpdate(ctx)

	time.Sleep(100 * time.Millisecond)

	err := mesh.PublishStatusUpdate(ctx, "mission-456", MissionStatusCompleted)
	if err != nil {
		t.Fatalf("unexpected error publishing status update: %v", err)
	}

	select {
	case msg := <-ch:
		expected := "mission-456:COMPLETED"
		if msg != expected {
			t.Errorf("expected '%s', got '%s'", expected, msg)
		}
	case <-time.After(1 * time.Second):
		t.Errorf("timeout waiting for message")
	}
}

func TestTeammateMesh_PublishAndSubscribeMail(t *testing.T) {
	s, mesh, client := setupTestMesh(t)
	defer s.Close()
	defer client.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	agentID := "agent-789"
	ch := mesh.SubscribeMail(ctx, agentID)

	time.Sleep(100 * time.Millisecond)

	err := mesh.PublishMail(ctx, agentID, "hello agent")
	if err != nil {
		t.Fatalf("unexpected error publishing mail: %v", err)
	}

	select {
	case msg := <-ch:
		if msg != "hello agent" {
			t.Errorf("expected 'hello agent', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Errorf("timeout waiting for message")
	}
}

func TestTeammateMesh_AcquireLock(t *testing.T) {
	s, mesh, client := setupTestMesh(t)
	defer s.Close()
	defer client.Close()

	ctx := context.Background()
	lock, err := mesh.AcquireLock(ctx, "test-resource", 1*time.Minute)
	if err != nil {
		t.Fatalf("unexpected error acquiring lock: %v", err)
	}
	if lock == nil {
		t.Fatalf("expected lock to be non-nil")
	}

	_, err = mesh.AcquireLock(ctx, "test-resource", 1*time.Minute)
	if err == nil {
		t.Errorf("expected error when acquiring already held lock")
	}

	ok, err := lock.UnlockContext(ctx)
	if err != nil || !ok {
		t.Errorf("failed to unlock: %v, ok=%v", err, ok)
	}

	lock2, err := mesh.AcquireLock(ctx, "test-resource", 1*time.Minute)
	if err != nil {
		t.Fatalf("unexpected error acquiring lock after release: %v", err)
	}
	defer lock2.UnlockContext(ctx)
}

func TestTeammateMesh_SubscribeContextCancel(t *testing.T) {
	s, mesh, client := setupTestMesh(t)
	defer s.Close()
	defer client.Close()

	ctx, cancel := context.WithCancel(context.Background())

	ch := mesh.SubscribeTaskCreated(ctx)

	cancel()

	select {
	case _, ok := <-ch:
		if ok {
			t.Errorf("expected channel to be closed, but it's open")
		}
	case <-time.After(1 * time.Second):
		t.Errorf("timeout waiting for channel to close")
	}

	// Trigger the ctx.Done() inside the second select inside the goroutine
	ctxBlocked, cancelBlocked := context.WithCancel(context.Background())
	chBlocked := mesh.SubscribeTaskCreated(ctxBlocked)
	time.Sleep(50 * time.Millisecond) // Let it start
	mesh.PublishTaskCreated(context.Background(), "block_test")
	time.Sleep(50 * time.Millisecond) // Let it read the message
	cancelBlocked() // Cancel it while it tries to send to unbuffered chBlocked
	<-chBlocked // Unblock it and consume any remaining message or close

	ctx2, cancel2 := context.WithCancel(context.Background())
	ch2 := mesh.SubscribeStatusUpdate(ctx2)
	time.Sleep(50 * time.Millisecond)
	mesh.PublishStatusUpdate(context.Background(), "block_test2", MissionStatusPending)
	time.Sleep(50 * time.Millisecond)
	cancel2()
	<-ch2

	ctx3, cancel3 := context.WithCancel(context.Background())
	ch3 := mesh.SubscribeMail(ctx3, "a")
	time.Sleep(50 * time.Millisecond)
	mesh.PublishMail(context.Background(), "a", "block_test3")
	time.Sleep(50 * time.Millisecond)
	cancel3()
	<-ch3
}

func TestTeammateMesh_SubscribeCloseRedis(t *testing.T) {
	s, mesh, client := setupTestMesh(t)
	defer s.Close()
	defer client.Close()

	ctx := context.Background()
	ch := mesh.SubscribeTaskCreated(ctx)
	ch2 := mesh.SubscribeStatusUpdate(ctx)
	ch3 := mesh.SubscribeMail(ctx, "a")

	client.Close()

	select {
	case <-ch:
	case <-time.After(1*time.Second):
	}
	select {
	case <-ch2:
	case <-time.After(1*time.Second):
	}
	select {
	case <-ch3:
	case <-time.After(1*time.Second):
	}
}
