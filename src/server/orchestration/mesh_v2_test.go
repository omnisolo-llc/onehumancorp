package orchestration

import (
	"context"
	"testing"
	"time"
)

func TestV2TeammateMesh(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	centrifuge := NewCentrifugeNode()
	mesh := NewV2TeammateMesh(centrifuge)

	channel := "test-channel"
	sub, err := mesh.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	msg := MeshMessage{
		AgentID:   "agent-1",
		Channel:   channel,
		EventType: "test-event",
		Data:      []byte(`{"foo":"bar"}`),
	}

	go func() {
		time.Sleep(100 * time.Millisecond)
		if err := mesh.Publish(ctx, msg); err != nil {
			t.Errorf("Failed to publish: %v", err)
		}
	}()

	select {
	case received := <-sub:
		if received.AgentID != msg.AgentID {
			t.Errorf("Expected agent %s, got %s", msg.AgentID, received.AgentID)
		}
	case <-ctx.Done():
		t.Fatal("Timeout waiting for message")
	}
}
