package orchestration

import (
	"context"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/proto"
)

func TestHubService_MeshAPIs(t *testing.T) {
	// Initialize memory mesh transport
	provider, err := db.NewSQLiteProvider(":memory:")
	require.NoError(t, err)

	_, err = provider.Exec(context.Background(), `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL,
			priority TEXT,
			agent_id TEXT,
			organization_id TEXT NOT NULL,
			payload TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN DEFAULT 0
		);
	`)
	require.NoError(t, err)

	mesh := NewMemoryMeshTransport(provider)
	hub := NewHub()
	hub.SetMeshTransport(mesh)

	service := NewHubServiceServer(hub)
	ctx := context.Background()

	// 1. Test AdvertiseCapabilities
	caps := pb.AgentCapabilities_builder{
		AgentId:            proto.String("test-agent-123"),
		SupportedSkills:    []string{"code", "test"},
		MaxConcurrentTasks: proto.Int32(5),
	}.Build()

	// Start a direct subscriber to verify it reaches the mesh
	capsChan, err := mesh.SubscribeCapabilities(ctx)
	require.NoError(t, err)

	resp, err := service.AdvertiseCapabilities(ctx, &caps)
	require.NoError(t, err)
	assert.True(t, resp.GetSuccess())

	select {
	case receivedCaps := <-capsChan:
		assert.Equal(t, caps.GetAgentId(), receivedCaps.GetAgentId())
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for capabilities message")
	}

	// 2. Test StreamMeshEvents (simulated)
	// Usually testing gRPC streams requires a full server setup, but we can verify the transport layer
	topic := "tasks"
	eventsChan, err := mesh.SubscribeMeshEvents(ctx, topic)
	require.NoError(t, err)

	payload := []byte(`{"status":"done"}`)
	err = mesh.BroadcastMeshEvent(ctx, topic, payload)
	require.NoError(t, err)

	select {
	case receivedBytes := <-eventsChan:
		assert.Equal(t, payload, receivedBytes)
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for mesh event message")
	}
}
