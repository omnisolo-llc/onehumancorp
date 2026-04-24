package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestV2TeammateMesh(t *testing.T) {
	ctx := context.Background()
	cn, err := NewCentrifugeNode()
	require.NoError(t, err)
	defer cn.Close()

	mesh := NewV2TeammateMesh(cn)
	require.NotNil(t, mesh)

	// BroadcastTask
	err = mesh.BroadcastTask(ctx, Task{
		TaskID:  "t-123",
		Action:  "READY",
		AgentID: "spiffe://onehumancorp.io/agent/x",
		Status:  "PENDING",
	})
	assert.NoError(t, err)

	// Unsupported methods
	_, err = mesh.SubscribeTasks(ctx)
	assert.ErrorContains(t, err, "not implemented")

	// BroadcastCoordination
	err = mesh.BroadcastCoordination(ctx, MeshMessage{
		AgentID:   "agent-z",
		Content:   "sync",
		Timestamp: time.Now(),
	})
	assert.NoError(t, err)

	_, err = mesh.SubscribeCoordination(ctx)
	assert.ErrorContains(t, err, "not implemented")
}

func TestV2TeammateMesh_NilHub(t *testing.T) {
	mesh := NewV2TeammateMesh(nil)

	err := mesh.BroadcastTask(context.Background(), Task{TaskID: "1"})
	assert.ErrorContains(t, err, "CentrifugeNode is nil")

	err = mesh.BroadcastCoordination(context.Background(), MeshMessage{})
	assert.ErrorContains(t, err, "CentrifugeNode is nil")
}
