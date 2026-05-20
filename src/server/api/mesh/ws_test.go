package mesh

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

func TestWSHandlerAuth(t *testing.T) {
	pubsub := NewMemoryPubSub()
	defer pubsub.Close()

	server := httptest.NewServer(WSHandler(pubsub))
	defer server.Close()

	url := "ws" + server.URL[4:] + "?topic=test"

	// Missing auth
	_, resp, err := websocket.DefaultDialer.Dial(url, nil)
	if err == nil {
		t.Fatal("Expected error when missing SPIFFE ID, got none")
	}
	if resp.StatusCode != http.StatusUnauthorized {
		t.Fatalf("Expected 401 Unauthorized, got %v", resp.StatusCode)
	}

	// With auth
	header := http.Header{}
	header.Add("X-Spiffe-ID", "spiffe://example.org/agent-1")
	conn, resp, err := websocket.DefaultDialer.Dial(url, header)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	defer conn.Close()

	if resp.StatusCode != http.StatusSwitchingProtocols {
		t.Fatalf("Expected 101 Switching Protocols, got %v", resp.StatusCode)
	}
}

func TestWSHandlerMessaging(t *testing.T) {
	pubsub := NewMemoryPubSub()
	defer pubsub.Close()

	server := httptest.NewServer(WSHandler(pubsub))
	defer server.Close()

	url := "ws" + server.URL[4:] + "?topic=test-topic"

	header := http.Header{}
	header.Add("X-Spiffe-ID", "spiffe://example.org/agent-1")
	conn, _, err := websocket.DefaultDialer.Dial(url, header)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	defer conn.Close()

    // allow subscription to complete
    time.Sleep(10 * time.Millisecond)

	msg := TeammateMeshEvent{
		EventType: "TASK_UPDATED",
		TaskID:    "task-123",
	}

	go func() {
		pubsub.Publish(context.Background(), "test-topic", msg)
	}()

	var received TeammateMeshEvent
	err = conn.ReadJSON(&received)
	if err != nil {
		t.Fatalf("Failed to read JSON: %v", err)
	}

	if received.TaskID != msg.TaskID {
		t.Fatalf("Expected task ID %v, got %v", msg.TaskID, received.TaskID)
	}
}
