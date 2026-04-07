package orchestration

import (
	"context"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
)

// We just test MemoryMeshTransport here as RedisMeshTransport would require a running Redis instance.
func TestMemoryMeshTransport_Capabilities(t *testing.T) {
	tm := NewMemoryMeshTransport(nil)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	capsChan, err := tm.SubscribeCapabilities(ctx)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	caps := pb.AgentCapabilities{
		AgentId: "agent-123",
		SupportedSkills: []string{"skill1", "skill2"},
	}

	err = tm.AdvertiseCapabilities(ctx, caps)
	if err != nil {
		t.Fatalf("Failed to broadcast: %v", err)
	}

	select {
	case received := <-capsChan:
		if received.GetAgentId() != caps.GetAgentId() {
			t.Errorf("Expected agent %s, got %s", caps.GetAgentId(), received.GetAgentId())
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for capability message")
	}
}

func TestMemoryMeshTransport_MeshEvents(t *testing.T) {
	tm := NewMemoryMeshTransport(nil)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	topic := "custom_topic"
	eventsChan, err := tm.SubscribeMeshEvents(ctx, topic)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	payload := []byte(`{"key": "value"}`)
	err = tm.BroadcastMeshEvent(ctx, topic, payload)
	if err != nil {
		t.Fatalf("Failed to broadcast: %v", err)
	}

	select {
	case received := <-eventsChan:
		if string(received) != string(payload) {
			t.Errorf("Expected payload %s, got %s", string(payload), string(received))
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for mesh event")
	}
}
