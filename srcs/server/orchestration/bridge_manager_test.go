package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/db"

)

func TestBridgeManager(t *testing.T) {


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

	dbProvider := db.NewTestProvider(t)
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
//


func TestBridgeManager_ConnectError(t *testing.T) {

	dbProvider := db.NewTestProvider(t)
	mt := NewMemoryMeshTransport(dbProvider)
	node := &CentrifugeNode{
		meshTransport: mt,
	}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	err := bm.Connect(ctx, "ws://invalid-url:9999", "remote-org-err", nil)
	if err == nil {
		t.Error("expected error when connecting to invalid URL")
	}
}

func TestBridgeManager_ForwardLoop_NodeNil(t *testing.T) {
    // Tests early return when node or meshTransport is nil
	bm := NewBridgeManager(nil, "mesh:tasks:shared", "local-org-1")
    ctx := context.Background()
    // It should just return without panicking
    bm.forwardLoop(ctx, "remote-org-1", nil)

    node := &CentrifugeNode{meshTransport: nil}
	bm2 := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")
    bm2.forwardLoop(ctx, "remote-org-1", nil)
}

func TestBridgeManager_RebroadcastRaw(t *testing.T) {
    // Test receiving a raw message from remote
    remoteReceived := make(chan []byte, 1)
	upgrader := websocket.Upgrader{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			t.Fatalf("Failed to upgrade websocket: %v", err)
		}
		defer conn.Close()

		conn.WriteMessage(websocket.TextMessage, []byte(`{"event":"raw_from_remote"}`))

		_, msg, err := conn.ReadMessage()
		if err == nil {
			remoteReceived <- msg
		}
	}))
	defer server.Close()

	wsURL := "ws" + server.URL[4:]

	dbProvider := db.NewTestProvider(t)
	mt := NewMemoryMeshTransport(dbProvider)
	node := &CentrifugeNode{meshTransport: mt}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	localSub, err := mt.SubscribeMeshEvents(ctx, "mesh:tasks:shared")
	if err != nil {
		t.Fatalf("Subscribe failed: %v", err)
	}

	err = bm.Connect(ctx, wsURL, "remote-org-raw", nil)
	if err != nil {
		t.Fatalf("Connect failed: %v", err)
	}

	select {
	case msg := <-localSub:
		expected := `{"origin_org_id":"remote-org-raw","payload":{"event":"raw_from_remote"}}`
		if string(msg) != expected {
			t.Errorf("Expected raw rebroadcast to have envelope %s, got %s", expected, string(msg))
		}
	case <-time.After(2 * time.Second):
		t.Fatal("Timeout waiting for remote raw message to reach local mesh")
	}
}

func TestBridgeManager_ForwardEnveloped(t *testing.T) {
	// Test forwarding a message that already has an envelope from us
    remoteReceived := make(chan []byte, 1)
	upgrader := websocket.Upgrader{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			t.Fatalf("Failed to upgrade websocket: %v", err)
		}
		defer conn.Close()
		_, msg, err := conn.ReadMessage()
		if err == nil {
			remoteReceived <- msg
		}
	}))
	defer server.Close()
	wsURL := "ws" + server.URL[4:]

	dbProvider := db.NewTestProvider(t)
	mt := NewMemoryMeshTransport(dbProvider)
	node := &CentrifugeNode{meshTransport: mt}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	err := bm.Connect(ctx, wsURL, "remote-org-env", nil)
	if err != nil {
		t.Fatalf("Connect failed: %v", err)
	}

    // Broadcast an already enveloped message from local
	err = mt.BroadcastMeshEvent(ctx, "mesh:tasks:shared", []byte(`{"origin_org_id":"local-org-1","payload":{"event":"already_enveloped"}}`))
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	select {
	case msg := <-remoteReceived:
		expected := `{"origin_org_id":"local-org-1","payload":{"event":"already_enveloped"}}`
		if string(msg) != expected {
			t.Errorf("Expected unchanged enveloped message %s, got %s", expected, string(msg))
		}
	case <-time.After(2 * time.Second):
		t.Fatal("Timeout waiting for remote to receive message")
	}
}

func TestBridgeManager_LoopPrevention(t *testing.T) {
    // Verify that messages originating from a remote org are NOT forwarded back
    remoteReceived := make(chan []byte, 1)
	upgrader := websocket.Upgrader{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			t.Fatalf("Failed to upgrade websocket: %v", err)
		}
		defer conn.Close()

        // Give time for the connection to establish before trying to read.
        time.Sleep(100 * time.Millisecond)
		_, msg, err := conn.ReadMessage()
		if err == nil {
			remoteReceived <- msg
		}
	}))
	defer server.Close()
	wsURL := "ws" + server.URL[4:]

	dbProvider := db.NewTestProvider(t)
	mt := NewMemoryMeshTransport(dbProvider)
	node := &CentrifugeNode{meshTransport: mt}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	err := bm.Connect(ctx, wsURL, "remote-org-loop", nil)
	if err != nil {
		t.Fatalf("Connect failed: %v", err)
	}

    // Broadcast an enveloped message from some OTHER remote org
	err = mt.BroadcastMeshEvent(ctx, "mesh:tasks:shared", []byte(`{"origin_org_id":"another-remote-org","payload":{"event":"should_not_forward"}}`))
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	select {
	case <-remoteReceived:
		t.Fatal("Remote received a message that should have been dropped due to loop prevention")
	case <-time.After(500 * time.Millisecond):
		// Success - nothing was forwarded
	}
}
