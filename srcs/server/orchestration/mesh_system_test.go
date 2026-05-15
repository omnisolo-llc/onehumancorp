package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"sync/atomic"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"onehumancorp/srcs/server/pb"
)

// We simulate a Standalone client (LocalTeammateMesh with an HTTP gateway mock) and a Cloud client (CentrifugeMesh).
func TestMeshSystem_BroadcastAndDiscover(t *testing.T) {
	// 1. Setup local mesh (representing standalone or server-side mesh state)
	localMesh := NewMemoryMeshTransport()

	// Simulate Cloud client
	var receivedCount int32
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Cloud client subscribes
	err := localMesh.Subscribe(ctx, "mesh:tasks", func(data []byte) {
		var msg MeshMessage
		err := json.Unmarshal(data, &msg)
		if err == nil && msg.EventType == "DEPLOY" {
			atomic.AddInt32(&receivedCount, 1)
		}
	})
	require.NoError(t, err)

	// Simulate HTTP gateway for Broadcast
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var msg MeshMessage
		json.NewDecoder(r.Body).Decode(&msg)
		data, _ := json.Marshal(msg)
		localMesh.Publish(r.Context(), "mesh:tasks", data)
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	dataPayload := json.RawMessage(`{"status": "IN_PROGRESS"}`)
	msg := MeshMessage{
		AgentID:   "cloud-agent",
		EventType: "DEPLOY",
		Data:      &dataPayload,
	}
	data, _ := json.Marshal(msg)
	req, _ := http.NewRequestWithContext(ctx, "POST", ts.URL, bytes.NewBuffer(data))
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	require.NoError(t, err)
	defer resp.Body.Close()

	agent := pb.Agent{
		ID:           "cloud-agent",
		Capabilities: []string{"deploy", "scale"},
		Status:       "IDLE",
	}
	localMesh.StartHeartbeat(ctx, agent)

	time.Sleep(15 * time.Millisecond) // Let heartbeat run at least once (we override for test)
	// We'll manually call AdvertiseCapabilities to ensure it's registered since ticker is 10s
	localMesh.AdvertiseCapabilities(ctx, agent)

	agents, err := localMesh.DiscoverAgents(ctx, "deploy")
	require.NoError(t, err)
	require.Len(t, agents, 1)
	assert.Equal(t, "cloud-agent", agents[0].ID)
}
