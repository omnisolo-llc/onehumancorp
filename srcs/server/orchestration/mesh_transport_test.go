package orchestration

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestMemoryMeshTransport_Capabilities(t *testing.T) {
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
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	capsChan, err := mesh.SubscribeCapabilities(ctx)
	require.NoError(t, err)

	caps := AgentCapabilities{
		AgentID:            "test-agent-1",
		SupportedSkills:    []string{"code", "test"},
		MaxConcurrentTasks: 5,
	}

	err = mesh.AdvertiseCapabilities(ctx, caps)
	require.NoError(t, err)

	select {
	case receivedCaps := <-capsChan:
		assert.Equal(t, caps.AgentID, receivedCaps.AgentID)
		assert.Equal(t, caps.SupportedSkills, receivedCaps.SupportedSkills)
		assert.Equal(t, caps.MaxConcurrentTasks, receivedCaps.MaxConcurrentTasks)
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for capabilities message")
	}
}

func TestMemoryMeshTransport_MeshEvents(t *testing.T) {
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
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	topic := "custom-topic"
	eventsChan, err := mesh.SubscribeMeshEvents(ctx, topic)
	require.NoError(t, err)

	payload := map[string]string{"foo": "bar"}
	payloadBytes, _ := json.Marshal(payload)

	err = mesh.BroadcastMeshEvent(ctx, topic, payloadBytes)
	require.NoError(t, err)

	select {
	case receivedBytes := <-eventsChan:
		var receivedPayload map[string]string
		err := json.Unmarshal(receivedBytes, &receivedPayload)
		require.NoError(t, err)
		assert.Equal(t, payload, receivedPayload)
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for mesh event message")
	}
}

func TestLocalTeammateMesh_Capabilities(t *testing.T) {
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

	mesh := NewLocalTeammateMesh(provider)
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	capsChan, err := mesh.SubscribeCapabilities(ctx)
	require.NoError(t, err)

	caps := pb.AgentCapabilities{
		AgentId:            "test-agent-1",
		SupportedSkills:    []string{"code", "test"},
		MaxConcurrentTasks: 5,
	}

	err = mesh.AdvertiseCapabilities(ctx, caps)
	require.NoError(t, err)

	select {
	case receivedCaps := <-capsChan:
		assert.Equal(t, caps.AgentId, receivedCaps.AgentId)
		assert.Equal(t, caps.SupportedSkills, receivedCaps.SupportedSkills)
		assert.Equal(t, caps.MaxConcurrentTasks, receivedCaps.MaxConcurrentTasks)
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for capabilities message")
	}
}

func TestLocalTeammateMesh_MeshEvents(t *testing.T) {
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

	mesh := NewLocalTeammateMesh(provider)
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	topic := "custom-topic"
	eventsChan, err := mesh.SubscribeMeshEvents(ctx, topic)
	require.NoError(t, err)

	payload := map[string]string{"foo": "bar"}
	payloadBytes, _ := json.Marshal(payload)

	err = mesh.BroadcastMeshEvent(ctx, topic, payloadBytes)
	require.NoError(t, err)

	select {
	case receivedBytes := <-eventsChan:
		var receivedPayload map[string]string
		err := json.Unmarshal(receivedBytes, &receivedPayload)
		require.NoError(t, err)
		assert.Equal(t, payload, receivedPayload)
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for mesh event message")
	}
}

// Minimal placeholder for Redis tests.
// Full integration test for Redis mesh requires a running Redis instance,
// which is usually handled in chaos_mesh_test.go or similar integration suites.
