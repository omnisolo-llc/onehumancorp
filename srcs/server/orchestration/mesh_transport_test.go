package orchestration

import (
	"context"
	"os"
	"testing"
	"time"
)

func TestMemoryMeshTransport_PublishSubscribe(t *testing.T) {
	mm := NewMemoryMeshTransport()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	topic := "test:topic"
	ch, err := mm.Subscribe(ctx, topic)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	payload := []byte("hello mesh")
	err = mm.Publish(ctx, topic, payload)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	select {
	case msg := <-ch:
		if string(msg) != string(payload) {
			t.Errorf("expected %q, got %q", payload, msg)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}

	// Test unsubscribe on cancel
	cancel()
	time.Sleep(100 * time.Millisecond) // Give goroutine time to cleanup

	mm.mu.RLock()
	subs := mm.subs[topic]
	mm.mu.RUnlock()

	if len(subs) != 0 {
		t.Errorf("expected 0 subscribers after cancel, got %d", len(subs))
	}
}

func TestRedisMeshTransport_PublishSubscribe(t *testing.T) {
	redisURL := os.Getenv("REDIS_URL")
	if redisURL == "" {
		t.Skip("skipping redis mesh transport test because REDIS_URL is not set")
	}

	rm, err := NewRedisMeshTransport(redisURL)
	if err != nil {
		t.Fatalf("failed to create redis mesh transport: %v", err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	topic := "test:topic:redis"
	ch, err := rm.Subscribe(ctx, topic)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Wait for subscription to establish
	time.Sleep(200 * time.Millisecond)

	payload := []byte("hello redis mesh")
	err = rm.Publish(ctx, topic, payload)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	select {
	case msg := <-ch:
		if string(msg) != string(payload) {
			t.Errorf("expected %q, got %q", payload, msg)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("timeout waiting for redis message")
	}
}
