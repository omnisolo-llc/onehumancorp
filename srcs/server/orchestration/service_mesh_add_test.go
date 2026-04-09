package orchestration

import (
	"context"
	"testing"

	pb "github.com/onehumancorp/mono/srcs/proto"
)

func TestHubServiceServer_AdvertiseCapabilities(t *testing.T) {
	// Setup a memory mesh transport and a HubServiceServer.
	mesh := NewMemoryMeshTransport(nil)
	hub := &Hub{} // Dummy hub; not directly used in AdvertiseCapabilities right now
	server := NewHubServiceServer(hub, mesh)

	ctx := context.Background()

	// Subscribe to capabilities to verify we receive what we advertise.
	capsCh, err := mesh.SubscribeCapabilities(ctx)
	if err != nil {
		t.Fatalf("failed to subscribe capabilities: %v", err)
	}

	// Create test capabilities.
	req := &pb.AgentCapabilities{
		AgentId: "test_agent",
		SupportedSkills: []string{"skill1", "skill2"},
		MaxConcurrentTasks: 5,
	}

	// Call the method under test.
	resp, err := server.AdvertiseCapabilities(ctx, req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if !resp.GetSuccess() {
		t.Errorf("expected success to be true")
	}

	// Verify the capability was broadcasted.
	select {
	case receivedCap := <-capsCh:
		if receivedCap.AgentId != "test_agent" {
			t.Errorf("expected agent ID test_agent, got %s", receivedCap.AgentId)
		}
	default:
		t.Errorf("expected capability to be available on channel")
	}
}
