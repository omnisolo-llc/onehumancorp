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

func TestTeammateMesh_StandaloneMode(t *testing.T) {
	mesh, err := NewTeammateMesh("")
	if err != nil {
		t.Fatalf("failed to create mesh: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		mesh.HandleWebSocket(w, r, "room-1")
	}))
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")

	// Connect client 1
	conn1, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("client 1 dial failed: %v", err)
	}
	defer conn1.Close()

	// Connect client 2
	conn2, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("client 2 dial failed: %v", err)
	}
	defer conn2.Close()

	// Wait briefly for subscriptions to register
	time.Sleep(100 * time.Millisecond)

	// Client 1 sends a message
	msg := `{"sender_id":"agent-1","role":"SWE","content":"hello"}`
	err = conn1.WriteMessage(websocket.TextMessage, []byte(msg))
	if err != nil {
		t.Fatalf("client 1 write failed: %v", err)
	}

	// Client 2 should receive the message
	conn2.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, p, err := conn2.ReadMessage()
	if err != nil {
		t.Fatalf("client 2 read failed: %v", err)
	}

	if !strings.Contains(string(p), "hello") {
		t.Errorf("expected payload to contain 'hello', got %s", string(p))
	}
}

func TestTeammateMesh_Publish(t *testing.T) {
	mesh, err := NewTeammateMesh("")
	if err != nil {
		t.Fatalf("failed to create mesh: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		mesh.HandleWebSocket(w, r, "room-1")
	}))
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")

	conn, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("dial failed: %v", err)
	}
	defer conn.Close()

	time.Sleep(100 * time.Millisecond)

	err = mesh.Publish(context.Background(), "room-1", `{"content":"direct publish"}`)
	if err != nil {
		t.Fatalf("publish failed: %v", err)
	}

	conn.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, p, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("read failed: %v", err)
	}

	if !strings.Contains(string(p), "direct publish") {
		t.Errorf("expected payload to contain 'direct publish', got %s", string(p))
	}
}
