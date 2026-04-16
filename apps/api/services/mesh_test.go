package services

import (
	"context"
	"testing"
	"time"
)

func TestMeshCoordinatorService_Local(t *testing.T) {
	svc := NewMeshCoordinatorService(nil)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	channel := "test-channel"

	sub, err := svc.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	msg := MeshMessage{
		ID:        "msg-1",
		Sender:    "agent-1",
		Channel:   channel,
		Content:   `{"test": "data"}`,
		CreatedAt: time.Now(),
	}

	if err := svc.Publish(ctx, msg); err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	select {
	case received := <-sub:
		if received.ID != msg.ID {
			t.Errorf("expected %s, got %s", msg.ID, received.ID)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}
}
