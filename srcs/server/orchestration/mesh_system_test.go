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

func TestMeshSystem_BroadcastAndDiscover(t *testing.T) {
	// 1. Setup local mesh (representing standalone or server-side mesh state)
	localMesh := NewLocalTeammateMesh()

	// Simulate client
	var receivedCount int32
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Client subscribes
	err := localMesh.Subscribe(ctx, "mesh:tasks", func(data []byte) {
		var msg MeshMessage
		err := json.Unmarshal(data, &msg)
		if err == nil && msg.Action == "DEPLOY" {
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

	// Direct publish using http client
	msg := MeshMessage{
		AgentID: "cloud-agent",
		Action:  "DEPLOY",
		Status:  "IN_PROGRESS",
	}
	data, _ := json.Marshal(msg)
	req, _ := http.NewRequestWithContext(ctx, "POST", ts.URL, bytes.NewBuffer(data))
	req.Header.Set("Content-Type", "application/json")
	resp, err := ts.Client().Do(req)
	require.NoError(t, err)
	resp.Body.Close()

	// Wait a bit
	time.Sleep(50 * time.Millisecond)

	assert.Equal(t, int32(1), atomic.LoadInt32(&receivedCount))

	// 2. Heartbeat & Discovery
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
