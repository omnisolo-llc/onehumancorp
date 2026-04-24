package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestBridgeManager(t *testing.T) {
	telemetry.InitTelemetry()

	// Mock server representing remote swarm
	remoteReceived := make(chan []byte, 1)
	upgrader := websocket.Upgrader{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			t.Fatalf("Failed to upgrade websocket: %v", err)
		}
		defer conn.Close()

		// Send a message back mimicking a remote enveloped message
		conn.WriteMessage(websocket.TextMessage, []byte(`{"origin_org_id":"remote-org-1", "payload":{"event":"hello_from_remote"}}`))

		// Read one message
		_, msg, err := conn.ReadMessage()
		if err == nil {
			remoteReceived <- msg
		}
	}))
	defer server.Close()

	wsURL := "ws" + server.URL[4:]

	dbProvider, _ := db.NewSqliteProvider("file::memory:?cache=shared")
	mt := NewMemoryMeshTransport(dbProvider)

	// Create a dummy CentrifugeNode for testing using the unexported fields
	node := &CentrifugeNode{
		meshTransport: mt,
	}

	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Subscribe to local mesh to verify incoming remote messages
	localSub, err := mt.SubscribeMeshEvents(ctx, "mesh:tasks:shared")
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	err = bm.Connect(ctx, wsURL, "remote-org-1", nil)
	if err != nil {
		t.Fatalf("Connect failed: %v", err)
	}

	status := bm.Status()
	if status["remote-org-1"] != "ACTIVE" {
		t.Errorf("Expected ACTIVE status, got %v", status)
	}

	// Wait for the remote message to reach the local mesh
	select {
	case msg := <-localSub:
		if string(msg) != `{"origin_org_id":"remote-org-1", "payload":{"event":"hello_from_remote"}}` {
			t.Errorf("Expected remote enveloped message, got %s", string(msg))
		}
	case <-time.After(2 * time.Second):
		t.Fatal("Timeout waiting for remote message to reach local mesh")
	}

	// Test forwarding to remote
	err = mt.BroadcastMeshEvent(ctx, "mesh:tasks:shared", []byte(`{"event":"hello_from_local"}`))
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	select {
	case msg := <-remoteReceived:
		// The forward loop should wrap it in an envelope
		expected := `{"origin_org_id":"local-org-1","payload":{"event":"hello_from_local"}}`
		if string(msg) != expected {
			t.Errorf("Expected enveloped %s, got %s", expected, string(msg))
		}
	case <-time.After(2 * time.Second):
		t.Fatal("Timeout waiting for remote to receive message")
	}
}
