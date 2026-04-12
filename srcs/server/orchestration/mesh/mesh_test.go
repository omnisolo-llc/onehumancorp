package mesh

import (
	"context"
	"sync"
	"testing"
	"time"
)

func TestLocalMesh_PubSub(t *testing.T) {
	mesh := NewLocalMesh()
	var wg sync.WaitGroup
	wg.Add(1)

	sub, err := mesh.Subscribe(context.Background(), "test-topic", func(msg []byte) {
		if string(msg) != "hello" {
			t.Errorf("expected hello, got %s", string(msg))
		}
		wg.Done()
	})
	if err != nil {
		t.Fatal(err)
	}
	defer sub.Unsubscribe()

	mesh.Publish(context.Background(), "test-topic", []byte("hello"))
	wg.Wait()
}

func TestLocalMesh_Lock(t *testing.T) {
	mesh := NewLocalMesh()
	acquired, err := mesh.AcquireLock(context.Background(), "my-lock", 1*time.Second)
	if err != nil || !acquired {
		t.Errorf("expected to acquire lock")
	}

	acquired2, _ := mesh.AcquireLock(context.Background(), "my-lock", 1*time.Second)
	if acquired2 {
		t.Errorf("expected lock to be busy")
	}

	mesh.ReleaseLock(context.Background(), "my-lock")
	acquired3, _ := mesh.AcquireLock(context.Background(), "my-lock", 1*time.Second)
	if !acquired3 {
		t.Errorf("expected to acquire lock after release")
	}
}

// Omitting Redis mock tests to avoid external miniredis dependency fetching issues in Bazel build
