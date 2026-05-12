package mesh

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"onehumancorp/srcs/server/orchestration"
	"onehumancorp/srcs/server/pb"
)

func TestMeshHandler_Broadcast(t *testing.T) {
	mesh := orchestration.NewLocalTeammateMesh()
	handler := NewMeshHandler(mesh)

	var receivedData []byte
	ch := make(chan struct{})

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	err := mesh.Subscribe(ctx, "mesh:broadcast", func(data []byte) {
		receivedData = data
		close(ch)
	})
	require.NoError(t, err)

	dataPayload := json.RawMessage(`{"status": "PENDING"}`)
	msg := orchestration.MeshMessage{
		AgentID:   "agent-1",
		EventType: "DO_WORK",
		Channel:   "mesh:broadcast",
		Data:      &dataPayload,
	}
	body, _ := json.Marshal(msg)

	req, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewBuffer(body))
	require.NoError(t, err)

	rr := httptest.NewRecorder()
	handler.Broadcast(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)

	select {
	case <-ch:
		var receivedMsg orchestration.MeshMessage
		err = json.Unmarshal(receivedData, &receivedMsg)
		require.NoError(t, err)
		assert.Equal(t, msg.AgentID, receivedMsg.AgentID)
		assert.Equal(t, msg.EventType, receivedMsg.EventType)
		assert.JSONEq(t, string(*msg.Data), string(*receivedMsg.Data))
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}

func TestMeshHandler_Capabilities(t *testing.T) {
	mesh := orchestration.NewLocalTeammateMesh()
	handler := NewMeshHandler(mesh)

	ctx := context.Background()
	agent := pb.Agent{
		ID:           "agent-1",
		Capabilities: []string{"coding", "testing"},
		Status:       "IDLE",
	}
	err := mesh.AdvertiseCapabilities(ctx, agent)
	require.NoError(t, err)

	req, err := http.NewRequest("GET", "/api/mesh/capabilities?skill=coding", nil)
	require.NoError(t, err)

	rr := httptest.NewRecorder()
	handler.Capabilities(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)

	var agents []pb.Agent
	err = json.Unmarshal(rr.Body.Bytes(), &agents)
	require.NoError(t, err)

	require.Len(t, agents, 1)
	assert.Equal(t, agent.ID, agents[0].ID)
}

func TestMeshHandler_Publish(t *testing.T) {
	mesh := orchestration.NewLocalTeammateMesh()
	handler := NewMeshHandler(mesh)

	var receivedData []byte
	ch := make(chan struct{})

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	err := mesh.Subscribe(ctx, "test-channel", func(data []byte) {
		receivedData = data
		close(ch)
	})
	require.NoError(t, err)

	body := []byte(`{"message": "hello"}`)
	req, err := http.NewRequest("POST", "/api/mesh/publish?channel=test-channel", bytes.NewBuffer(body))
	require.NoError(t, err)

	rr := httptest.NewRecorder()
	handler.Publish(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)

	select {
	case <-ch:
		assert.Equal(t, string(body), string(receivedData))
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}

func TestMeshHandler_Subscribe(t *testing.T) {
	mesh := orchestration.NewLocalTeammateMesh()
	handler := NewMeshHandler(mesh)

	server := httptest.NewServer(http.HandlerFunc(handler.Subscribe))
	defer server.Close()

	wsURL := "ws" + server.URL[4:] + "?channel=test-channel"

	// Connect via websocket
	conn, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	require.NoError(t, err)
	defer conn.Close()

	// Wait for the server to register the subscriber
	time.Sleep(100 * time.Millisecond)

	// Publish to the mesh
	ctx := context.Background()
	body := []byte(`{"message": "hello from mesh"}`)
	err = mesh.Publish(ctx, "test-channel", body)
	require.NoError(t, err)

	// Read from websocket
	_ = conn.SetReadDeadline(time.Now().Add(time.Second * 5))
	_, message, err := conn.ReadMessage()
	require.NoError(t, err)
	assert.Equal(t, string(body), string(message))
}
