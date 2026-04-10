package hub

import (
	"context"
	"testing"
	"time"
)

func TestTeammateMeshService(t *testing.T) {
	svc := NewTeammateMeshService()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	topic := "test-topic"
	ch, err := svc.Subscribe(ctx, topic)
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	msg := HubMessage{ID: "msg1", Payload: "test payload"}
	err = svc.PublishMessage(ctx, topic, msg)
	if err != nil {
		t.Fatalf("PublishMessage failed: %v", err)
	}

	select {
	case receivedMsg := <-ch:
		if receivedMsg.ID != "msg1" {
			t.Errorf("Expected msg1, got %s", receivedMsg.ID)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}
