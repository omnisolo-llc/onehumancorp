package mesh

import (
	"context"
	"errors"
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

func TestLocalMesh_PubSub_PublishNoSubscribers(t *testing.T) {
    mesh := NewLocalMesh()
    ctx := context.Background()

    err := mesh.Publish(ctx, "no-subs-topic", []byte("data"))
    if err != nil {
        t.Fatalf("Publishing to no subscribers should return nil, got %v", err)
    }
}

func TestLocalMesh_PubSub_PublishContextCancelled(t *testing.T) {
    mesh := NewLocalMesh()
    ctx, cancel := context.WithCancel(context.Background())

    // Sature the buffer
    _, _ = mesh.Subscribe(context.Background(), "full-topic", func(msg []byte) {
        time.Sleep(1 * time.Second)
    })

    for i := 0; i < 150; i++ {
         _ = mesh.Publish(context.Background(), "full-topic", []byte("data"))
    }

    cancel() // cancel immediately
    err := mesh.Publish(ctx, "full-topic", []byte("data"))
    if !errors.Is(err, context.Canceled) {
        t.Fatalf("Expected context canceled error when channel is full and context is canceled, got %v", err)
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

	// Release with wrong token
	err = mesh.ReleaseLock(ctx, "test-lock", "wrong-token")
	if err == nil {
		t.Fatalf("Expected error when releasing with wrong token")
	}

	// Release
	err = mesh.ReleaseLock(ctx, "test-lock", token)
	if err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

    // Release again (should fail)
    err = mesh.ReleaseLock(ctx, "test-lock", token)
    if err == nil {
        t.Fatalf("Expected error when releasing already released lock")
    }

	// Try again after release
	token2, acquired, err := mesh.AcquireLock(ctx, "test-lock", 2*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if !acquired {
		t.Errorf("Expected lock to be acquired after release")
	}
    if token == token2 {
        t.Errorf("Expected new token")
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

	// Release with wrong token
	err = mesh.ReleaseLock(ctx, "test-lock", "wrong-token")
	if err == nil {
		t.Fatalf("Expected error when releasing with wrong token")
	}

	// Release
	err = mesh.ReleaseLock(ctx, "test-lock", token)
	if err != nil {
		t.Fatalf("Failed to release lock: %v", err)
	}

    // Release again (should fail)
    err = mesh.ReleaseLock(ctx, "test-lock", token)
    if err == nil {
        t.Fatalf("Expected error when releasing already released lock")
    }

	// Try again after release
	token2, acquired, err := mesh.AcquireLock(ctx, "test-lock", 2*time.Second)
	if err != nil {
		t.Fatalf("Failed to acquire lock: %v", err)
	}
	if !acquired {
		t.Errorf("Expected lock to be acquired after release")
	}
    if token == token2 {
        t.Errorf("Expected new token")
    }
}
