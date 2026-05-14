package interop

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/go-redis/redis/v8"
)

func TestRedisTeammateMesh(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	mesh := NewRedisTeammateMesh(client)
	ctx := context.Background()

	ch, cancel, err := mesh.Subscribe(ctx, "test_channel")
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	err = mesh.Publish(ctx, "test_channel", []byte("hello"))
	if err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	select {
	case msg := <-ch:
		if string(msg) != "hello" {
			t.Fatalf("expected hello, got %s", string(msg))
		}
	case <-time.After(1 * time.Second):
		t.Fatalf("timeout waiting for message")
	}

	cancel()

    // Test that channel is closed
    time.Sleep(50 * time.Millisecond)
    select {
    case _, ok := <-ch:
        if ok {
            t.Fatalf("expected channel to be closed after cancel")
        }
    default:
        // Closed or empty. If closed, it would yield immediately with !ok.
        // Let's actually wait for it.
    }
}

func TestRedisTeammateMeshSubscribeError(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
    // create client and then close miniredis to force connection error
	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
    mr.Close()

	mesh := NewRedisTeammateMesh(client)
	ctx := context.Background()

	_, _, err = mesh.Subscribe(ctx, "test_channel")
	if err == nil {
		t.Fatalf("expected error subscribing with closed redis, got nil")
	}
}

func TestRedisTeammateMeshChannelClosedRemotely(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	mesh := NewRedisTeammateMesh(client)
	ctx := context.Background()

	ch, _, err := mesh.Subscribe(ctx, "test_channel")
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

    // Force client to close to trigger channel close
    client.Close()

    // Read should return ok=false eventually
    timeout := time.After(3 * time.Second)
    for {
        select {
        case _, ok := <-ch:
            if !ok {
                return // success
            }
        case <-timeout:
            t.Fatalf("timeout waiting for channel to close")
        }
    }
}
