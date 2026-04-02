package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

func TestMeshTransport(t *testing.T) {
	h := NewHub()
	mt := NewMeshTransport(h)

	server := httptest.NewServer(http.HandlerFunc(mt.HandleWS))
	defer server.Close()

	url := "ws" + strings.TrimPrefix(server.URL, "http")

	// Connect Client 1
	dialer := websocket.Dialer{HandshakeTimeout: 2 * time.Second}
	conn1, _, err := dialer.Dial(url+"?agent_id=agent-1", nil)
	if err != nil {
		t.Fatalf("Failed to connect client 1: %v", err)
	}
	defer conn1.Close()

	// Connect Client 2
	conn2, _, err := dialer.Dial(url+"?agent_id=agent-2", nil)
	if err != nil {
		t.Fatalf("Failed to connect client 2: %v", err)
	}
	defer conn2.Close()

	// Wait for connections to be registered
	time.Sleep(100 * time.Millisecond)

	// Send message from Client 1
	msg := `{"type":"TASK_BROADCAST","content":"Hello from 1"}`
	if err := conn1.WriteMessage(websocket.TextMessage, []byte(msg)); err != nil {
		t.Fatalf("Failed to send message: %v", err)
	}

	// Read message on Client 2
	conn2.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, msgBytes, err := conn2.ReadMessage()
	if err != nil {
		t.Fatalf("Failed to read message on client 2: %v", err)
	}

	if string(msgBytes) != msg {
		t.Fatalf("Expected %s, got %s", msg, string(msgBytes))
	}
}
