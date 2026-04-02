package orchestration

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

func TestTeammateMesh(t *testing.T) {
	mesh := NewTeammateMesh()

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Mock routing logic from main app
		if strings.HasPrefix(r.URL.Path, "/api/v1/mesh/rooms/") {
			mesh.ServeHTTP(w, r)
		} else {
			http.NotFound(w, r)
		}
	}))
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "/api/v1/mesh/rooms/test-room"

	conn1, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("dial conn1 failed: %v", err)
	}
	defer conn1.Close()

	conn2, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("dial conn2 failed: %v", err)
	}
	defer conn2.Close()

	// Wait for connections to be registered
	time.Sleep(50 * time.Millisecond)

	msg := MeshMessage{
		SenderID:  "agent-1",
		Role:      "SWE",
		Content:   "Hello Mesh",
		Timestamp: time.Now(),
	}

	payload, _ := json.Marshal(msg)
	if err := conn1.WriteMessage(websocket.TextMessage, payload); err != nil {
		t.Fatalf("conn1 write failed: %v", err)
	}

	// conn2 should receive it
	_, readPayload, err := conn2.ReadMessage()
	if err != nil {
		t.Fatalf("conn2 read failed: %v", err)
	}

	var readMsg MeshMessage
	if err := json.Unmarshal(readPayload, &readMsg); err != nil {
		t.Fatalf("unmarshal failed: %v", err)
	}

	if readMsg.Content != "Hello Mesh" {
		t.Errorf("expected Hello Mesh, got %s", readMsg.Content)
	}

	// Test fallback broadcasting unparsable JSON
	if err := conn1.WriteMessage(websocket.TextMessage, []byte("plain text")); err != nil {
		t.Fatalf("conn1 write text failed: %v", err)
	}

	_, readPayload2, err := conn2.ReadMessage()
	if err != nil {
		t.Fatalf("conn2 read text failed: %v", err)
	}

	var readMsg2 MeshMessage
	if err := json.Unmarshal(readPayload2, &readMsg2); err != nil {
		t.Fatalf("unmarshal text failed: %v", err)
	}

	if readMsg2.Content != "plain text" {
		t.Errorf("expected plain text content, got %s", readMsg2.Content)
	}
}

func TestTeammateMesh_MissingRoomID(t *testing.T) {
	mesh := NewTeammateMesh()

	req, _ := http.NewRequest("GET", "/api/v1/mesh/rooms/", nil)
	rr := httptest.NewRecorder()

	mesh.ServeHTTP(rr, req)

	if rr.Code != http.StatusBadRequest {
		t.Errorf("expected 400 Bad Request for missing room ID, got %v", rr.Code)
	}
}
