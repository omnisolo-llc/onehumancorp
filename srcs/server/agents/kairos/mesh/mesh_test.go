package mesh

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestLocalMesh_PubSub(t *testing.T) {
	mesh := NewLocalMesh()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	ch, err := mesh.Subscribe(ctx, ChannelTaskCreated)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	go func() {
		err := mesh.Publish(ctx, ChannelTaskCreated, "hello world")
		if err != nil {
			t.Errorf("failed to publish: %v", err)
		}
	}()

	select {
	case msg := <-ch:
		if msg != "hello world" {
			t.Errorf("expected 'hello world', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}
}

func TestLocalMesh_Lock(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	unlock1, err := mesh.AcquireLock(ctx, "test_lock")
	if err != nil {
		t.Fatalf("failed to acquire lock 1: %v", err)
	}

	lockedChan := make(chan struct{})
	go func() {
		unlock2, _ := mesh.AcquireLock(ctx, "test_lock")
		unlock2()
		close(lockedChan)
	}()

	select {
	case <-lockedChan:
		t.Fatal("lock 2 acquired before lock 1 was released")
	case <-time.After(100 * time.Millisecond):
		// Expected, lock 2 is waiting
	}

	unlock1()

	select {
	case <-lockedChan:
		// Lock 2 was acquired and released
	case <-time.After(100 * time.Millisecond):
		t.Fatal("lock 2 was not acquired after lock 1 was released")
	}
}

func TestRedisMesh_PubSub(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{Addr: mr.Addr()})
	mesh := NewRedisMesh(client)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	ch, err := mesh.Subscribe(ctx, ChannelStatusUpdate)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	// Small delay to ensure subscription is active in redis
	time.Sleep(50 * time.Millisecond)

	go func() {
		err := mesh.Publish(ctx, ChannelStatusUpdate, "status OK")
		if err != nil {
			t.Errorf("failed to publish: %v", err)
		}
	}()

	select {
	case msg := <-ch:
		if msg != "status OK" {
			t.Errorf("expected 'status OK', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}
}

func TestRedisMesh_Lock(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{Addr: mr.Addr()})
	mesh := NewRedisMesh(client)

	ctx := context.Background()

	unlock1, err := mesh.AcquireLock(ctx, "test_lock_redis")
	if err != nil {
		t.Fatalf("failed to acquire lock 1: %v", err)
	}

	lockedChan := make(chan struct{})
	go func() {
		unlock2, _ := mesh.AcquireLock(ctx, "test_lock_redis")
		unlock2()
		close(lockedChan)
	}()

	select {
	case <-lockedChan:
		t.Fatal("lock 2 acquired before lock 1 was released")
	case <-time.After(200 * time.Millisecond):
		// Expected, lock 2 is waiting (polling)
	}

	unlock1()

	select {
	case <-lockedChan:
		// Lock 2 was acquired and released
	case <-time.After(500 * time.Millisecond):
		t.Fatal("lock 2 was not acquired after lock 1 was released")
	}
}
