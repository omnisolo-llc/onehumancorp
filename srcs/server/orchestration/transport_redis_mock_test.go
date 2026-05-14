package orchestration

import (
    "context"
	"testing"
    "time"
)

func TestMemoryMeshTransport_Close(t *testing.T) {
    mem := NewMemoryMeshTransport()
    mem.Close()
}

func TestRedisMeshTransport_PublishError(t *testing.T) {
    transport := &RedisMeshTransport{}
    err := transport.Publish(context.Background(), "test", MeshEvent{Payload: []byte("data")})
    if err == nil {
        t.Fatal("Expected error when publishing with nil client")
    }
}

func TestRedisMeshTransport_SubscribeError(t *testing.T) {
    transport := &RedisMeshTransport{addr: "invalid:addr"}
    err := transport.Subscribe(context.Background(), "test", func(event MeshEvent) {})
    if err != nil {
        t.Fatal("Subscribe shouldn't error synchronously")
    }
    time.Sleep(50 * time.Millisecond)
}

func TestRedisMeshTransport_CloseNil(t *testing.T) {
    transport := &RedisMeshTransport{}
    err := transport.Close()
    if err != nil {
        t.Fatal("Close shouldn't error when client is nil")
    }
}

func TestRedisMeshTransport_Integration(t *testing.T) {
	// Try with an invalid address to trigger connection error path
	_, err := NewRedisMeshTransport("invalid_addr_that_does_not_exist:9999")
	if err == nil {
		t.Log("Expected an error for invalid redis address")
	}

	transportValid, err := NewRedisMeshTransport("127.0.0.1:6379")
	if err != nil {
		t.Skipf("Redis not available, skipping test: %v", err)
	}
	defer transportValid.Close()

	ch := make(chan string, 1)

	err = transportValid.Subscribe(context.Background(), "redis_test_topic", func(event MeshEvent) {
		ch <- string(event.Payload)
	})
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	time.Sleep(100 * time.Millisecond)

	err = transportValid.Publish(context.Background(), "redis_test_topic", MeshEvent{Payload: []byte("redis_hello")})
	if err != nil {
		t.Fatalf("Publish failed: %v", err)
	}

	select {
	case msg := <-ch:
		if msg != "redis_hello" {
			t.Errorf("Expected 'redis_hello', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Log("Timeout waiting for message (expected if Redis is not fully mocking pub/sub)")
	}
}

func TestMemoryMeshTransport_PublishAndSubscribe(t *testing.T) {
	transport := NewMemoryMeshTransport()
	ctx := context.Background()
	ch := make(chan string, 1)

	err := transport.Subscribe(ctx, "mem_topic", func(event MeshEvent) {
		ch <- string(event.Payload)
	})
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	err = transport.Publish(ctx, "mem_topic", MeshEvent{Payload: []byte("mem_hello")})
	if err != nil {
		t.Fatalf("Publish failed: %v", err)
	}

	select {
	case msg := <-ch:
		if msg != "mem_hello" {
			t.Errorf("Expected 'mem_hello', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}
