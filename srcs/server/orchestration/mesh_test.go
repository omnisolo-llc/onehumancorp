package orchestration

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

func TestMeshManager_BroadcastAndSubscribe(t *testing.T) {
	manager := NewMeshManager()

	// Setup mock server for websocket
	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/mesh/rooms/{room_id}", manager.SubscribeHandler)
	mux.HandleFunc("/api/v1/mesh/rooms/{room_id}/messages", manager.PublishHandler)
	server := httptest.NewServer(mux)
	defer server.Close()

	// Connect via websocket
	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "/api/v1/mesh/rooms/room-1"
	conn, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("failed to connect to websocket: %v", err)
	}
	defer conn.Close()

	// Publish via HTTP
	publishURL := server.URL + "/api/v1/mesh/rooms/room-1/messages"
	msg := MeshMessage{
		SenderID:  "agent-1",
		Role:      "SWE",
		Content:   "Hello Mesh",
		Timestamp: time.Now().UTC(),
	}
	payload, _ := json.Marshal(msg)
	resp, err := http.Post(publishURL, "application/json", bytes.NewBuffer(payload))
	if err != nil {
		t.Fatalf("failed to publish message: %v", err)
	}
	if resp.StatusCode != http.StatusOK {
		t.Fatalf("expected status OK, got %d", resp.StatusCode)
	}

	// Read message from websocket
	conn.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, p, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("failed to read message from websocket: %v", err)
	}

	var receivedMsg MeshMessage
	if err := json.Unmarshal(p, &receivedMsg); err != nil {
		t.Fatalf("failed to unmarshal received message: %v", err)
	}

	if receivedMsg.Content != "Hello Mesh" {
		t.Errorf("expected content 'Hello Mesh', got '%s'", receivedMsg.Content)
	}
	if receivedMsg.SenderID != "agent-1" {
		t.Errorf("expected sender 'agent-1', got '%s'", receivedMsg.SenderID)
	}
}
