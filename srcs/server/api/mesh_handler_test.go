package api

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestMeshHandler_HandleBroadcast(t *testing.T) {
	trans := orchestration.NewMemoryMeshTransport()
	handler := NewMeshHandler(trans)

	payload := map[string]interface{}{
		"topic": "test-topic",
		"message": map[string]interface{}{
			"agent_id": "test-agent",
			"action":   "test-action",
		},
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", bytes.NewReader(body))
	rec := httptest.NewRecorder()

	handler.HandleBroadcast(rec, req)

	if rec.Code != http.StatusOK {
		t.Errorf("expected 200, got %d", rec.Code)
	}
}

func TestMeshHandler_HandleSubscribe(t *testing.T) {
	trans := orchestration.NewMemoryMeshTransport()
	handler := NewMeshHandler(trans)

	server := httptest.NewServer(http.HandlerFunc(handler.HandleSubscribe))
	defer server.Close()

	url := "ws" + server.URL[4:] + "?topic=test-topic"

	conn, _, err := websocket.DefaultDialer.Dial(url, nil)
	if err != nil {
		t.Fatalf("failed to dial: %v", err)
	}
	defer conn.Close()

	time.Sleep(100 * time.Millisecond) // Give the websocket handler a moment to subscribe

	msg := orchestration.MeshMessage{
		AgentID: "test-agent",
		Action:  "test-action",
	}
	err = trans.Publish(context.Background(), "test-topic", msg)
	if err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	conn.SetReadDeadline(time.Now().Add(2 * time.Second))
	var received orchestration.MeshMessage
	err = conn.ReadJSON(&received)
	if err != nil {
		t.Fatalf("failed to read json: %v", err)
	}

	if received.AgentID != "test-agent" {
		t.Errorf("expected test-agent, got %s", received.AgentID)
	}
}
