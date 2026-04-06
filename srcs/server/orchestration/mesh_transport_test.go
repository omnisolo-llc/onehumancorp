package orchestration

import (
	"context"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
)

func TestMemoryMeshTransport(t *testing.T) {
	tm := NewMemoryMeshTransport()
	ctx := context.Background()

	ch, err := tm.SubscribeEvents(ctx, "test-topic")
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	event := &pb.MeshEvent{}
	event.SetEventId("123")
	event.SetTopic("test-topic")
	event.SetPayload([]byte("hello"))
	event.SetTimestamp(time.Now().Unix())

	err = tm.PublishEvent(ctx, "test-topic", event)
	if err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	select {
	case received := <-ch:
		if received.GetEventId() != "123" {
			t.Errorf("expected event id '123', got %s", received.GetEventId())
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for event")
	}
}
