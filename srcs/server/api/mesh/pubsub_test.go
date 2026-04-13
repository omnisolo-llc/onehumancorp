package mesh

import (
	"context"
	"testing"
	"time"
)

func TestMemoryPubSub(t *testing.T) {
	pubsub := NewMemoryPubSub()
	ctx := context.Background()

	ch, unsubscribe, err := pubsub.Subscribe(ctx, "test-topic")
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	err = pubsub.Publish(ctx, "test-topic", []byte("hello"))
	if err != nil {
		t.Fatalf("Publish failed: %v", err)
	}

	select {
	case msg := <-ch:
		if string(msg) != "hello" {
			t.Errorf("Expected 'hello', got '%s'", string(msg))
		}
	case <-time.After(1 * time.Second):
		t.Error("Timeout waiting for message")
	}

	err = unsubscribe()
	if err != nil {
		t.Errorf("Unsubscribe failed: %v", err)
	}
}
