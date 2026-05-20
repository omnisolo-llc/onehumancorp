package mesh

import (
	"context"
	"testing"
	"time"
)

func TestMemoryPubSub(t *testing.T) {
	pubsub := NewMemoryPubSub()
	defer pubsub.Close()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	topic := "test-topic"
	ch, err := pubsub.Subscribe(ctx, topic)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	msg := TeammateMeshEvent{
		EventType: "TASK_UPDATED",
		TaskID:    "task-123",
	}

	go func() {
		err := pubsub.Publish(ctx, topic, msg)
		if err != nil {
			t.Errorf("Failed to publish: %v", err)
		}
	}()

	select {
	case received := <-ch:
		if received.TaskID != msg.TaskID {
			t.Errorf("Expected task ID %s, got %s", msg.TaskID, received.TaskID)
		}
	case <-time.After(time.Second):
		t.Fatal("Timeout waiting for message")
	}
}
