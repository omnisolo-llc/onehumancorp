package mesh

import (
	"context"
	"testing"
	"time"
)

func TestMemoryPubSub(t *testing.T) {
	pubsub := NewMemoryPubSub()
	topic := "test-topic"

	ch, err := pubsub.Subscribe(context.Background(), topic)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	msg := []byte("hello")
	if err := pubsub.Publish(context.Background(), topic, msg); err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	select {
	case received := <-ch:
		if string(received) != string(msg) {
			t.Errorf("Expected %s, got %s", string(msg), string(received))
		}
	case <-time.After(time.Second):
		t.Error("Timed out waiting for message")
	}
}
