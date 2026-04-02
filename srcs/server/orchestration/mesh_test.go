package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/mux"
	"github.com/gorilla/websocket"
	"github.com/stretchr/testify/assert"
)

func TestMeshManager_Local(t *testing.T) {
	os.Unsetenv("OHC_MULTITENANT")

	mm := NewMeshManager(nil)
	router := mux.NewRouter()
	router.HandleFunc("/api/v1/mesh/rooms/{room_id}", mm.HandleSubscribe).Methods("GET")
	router.HandleFunc("/api/v1/mesh/rooms/{room_id}/messages", mm.HandlePublish).Methods("POST")

	server := httptest.NewServer(router)
	defer server.Close()

	// 1. Connect WebSocket Subscriber
	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "/api/v1/mesh/rooms/test-room"
	ws, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	assert.NoError(t, err)
	defer ws.Close()

	// Let the subscriber register
	time.Sleep(100 * time.Millisecond)

	// 2. Publish Message via HTTP
	msg := MeshMessage{
		SenderID:  "agent-123",
		Role:      "SWE",
		Content:   "Hello from test",
		Timestamp: time.Now(),
	}
	body, _ := json.Marshal(msg)

	resp, err := http.Post(server.URL+"/api/v1/mesh/rooms/test-room/messages", "application/json", bytes.NewBuffer(body))
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, resp.StatusCode)

	// 3. Receive Message on WebSocket
	var receivedMsg MeshMessage
	err = ws.SetReadDeadline(time.Now().Add(2 * time.Second))
	assert.NoError(t, err)

	err = ws.ReadJSON(&receivedMsg)
	assert.NoError(t, err)

	assert.Equal(t, "agent-123", receivedMsg.SenderID)
	assert.Equal(t, "SWE", receivedMsg.Role)
	assert.Equal(t, "Hello from test", receivedMsg.Content)
}

func TestMeshManager_Cloud(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	mr, redisClient := setupRedis(t)
	defer mr.Close()

	mm := NewMeshManager(redisClient)
	router := mux.NewRouter()
	router.HandleFunc("/api/v1/mesh/rooms/{room_id}", mm.HandleSubscribe).Methods("GET")
	router.HandleFunc("/api/v1/mesh/rooms/{room_id}/messages", mm.HandlePublish).Methods("POST")

	server := httptest.NewServer(router)
	defer server.Close()

	// 1. Connect WebSocket Subscriber
	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "/api/v1/mesh/rooms/test-room-cloud"
	ws, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	assert.NoError(t, err)
	defer ws.Close()

	// Let the subscriber register and redis pubsub connect
	time.Sleep(500 * time.Millisecond)

	// 2. Publish Message directly via Redis or HTTP
	msg := MeshMessage{
		SenderID:  "agent-cloud",
		Role:      "PM",
		Content:   "Hello from cloud",
		Timestamp: time.Now(),
	}
	body, _ := json.Marshal(msg)

	resp, err := http.Post(server.URL+"/api/v1/mesh/rooms/test-room-cloud/messages", "application/json", bytes.NewBuffer(body))
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, resp.StatusCode)

	// 3. Receive Message on WebSocket
	var receivedMsg MeshMessage
	err = ws.SetReadDeadline(time.Now().Add(2 * time.Second))
	assert.NoError(t, err)

	err = ws.ReadJSON(&receivedMsg)
	assert.NoError(t, err)

	assert.Equal(t, "agent-cloud", receivedMsg.SenderID)
	assert.Equal(t, "PM", receivedMsg.Role)
	assert.Equal(t, "Hello from cloud", receivedMsg.Content)
}
