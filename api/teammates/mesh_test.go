package teammates

import (
	"context"
	"testing"
	"time"
)

func TestLocalMesh_PubSub(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	received := make(chan string, 1)
	sub, err := mesh.Subscribe(ctx, "test_topic", func(msg []byte) {
		received <- string(msg)
	})
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	err = mesh.Publish(ctx, "test_topic", []byte("hello"))
	if err != nil {
		t.Fatalf("Publish failed: %v", err)
	}

	select {
	case msg := <-received:
		if msg != "hello" {
			t.Errorf("Expected 'hello', got '%s'", msg)
		}
	case <-time.After(time.Second):
		t.Fatal("Timeout waiting for message")
	}

	sub.Close()
}

func TestLocalMesh_Lock(t *testing.T) {
	mesh := NewLocalMesh()
	ctx := context.Background()

	ok, err := mesh.AcquireLock(ctx, "my_lock", time.Second)
	if err != nil || !ok {
		t.Fatalf("Failed to acquire lock: %v, %v", ok, err)
	}

	ok, err = mesh.AcquireLock(ctx, "my_lock", time.Second)
	if err != nil || ok {
		t.Fatalf("Should not acquire lock twice: %v, %v", ok, err)
	}

	err = mesh.ReleaseLock(ctx, "my_lock")
	if err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

	ok, err = mesh.AcquireLock(ctx, "my_lock", time.Second)
	if err != nil || !ok {
		t.Fatalf("Failed to acquire lock after release: %v, %v", ok, err)
	}
}
