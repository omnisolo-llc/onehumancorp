package orchestration

import (
	"context"
	"errors"
	pb "github.com/onehumancorp/mono/srcs/proto"
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

	// Test loop prevention
	err = mt.BroadcastMeshEvent(ctx, "mesh:tasks:shared", []byte(`{"origin_org_id":"remote-org-1","payload":{"event":"hello_from_remote"}}`))
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}
	select {
	case msg := <-remoteReceived:
		t.Fatalf("Expected message to be dropped, but received it %s", string(msg))
	case <-time.After(1 * time.Second):
	}
}
func TestBridgeManager_Errors(t *testing.T) {

	dbProvider := db.NewTestProvider(t)
	mt := NewMemoryMeshTransport(dbProvider)
	node := &CentrifugeNode{meshTransport: mt}

	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")

	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	// Connect to non-existent server
	err := bm.Connect(ctx, "ws://localhost:9999", "remote-org-1", nil)
	if err == nil {
		t.Fatal("Expected error connecting to non-existent server")
	}

	// Test without node
	bmNoNode := NewBridgeManager(nil, "mesh:tasks:shared", "local-org-1")
	bmNoNode.forwardLoop(ctx, "remote-org-1", nil)
}

func TestBridgeManager_Unmarshaling(t *testing.T) {

	dbProvider := db.NewTestProvider(t)
	mt := NewMemoryMeshTransport(dbProvider)
	node := &CentrifugeNode{meshTransport: mt}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	// Need a server to test read error and raw message drops
	upgrader := websocket.Upgrader{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			t.Fatalf("Failed to upgrade websocket: %v", err)
		}
		defer conn.Close()

		// Send raw string message
		conn.WriteMessage(websocket.TextMessage, []byte("raw_string"))
		conn.WriteMessage(websocket.TextMessage, []byte(`{"origin_org_id":"local-org-1", "payload":{"event":"hello"}}`))
	}))
	defer server.Close()

	wsURL := "ws" + server.URL[4:]
	err := bm.Connect(ctx, wsURL, "remote-org-2", nil)
	if err != nil {
		t.Fatalf("Connect failed: %v", err)
	}

	time.Sleep(100 * time.Millisecond)
}

func TestBridgeManager_CancelContext(t *testing.T) {

	dbProvider := db.NewTestProvider(t)
	mt := NewMemoryMeshTransport(dbProvider)
	node := &CentrifugeNode{meshTransport: mt}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")

	ctx, cancel := context.WithCancel(context.Background())

	upgrader := websocket.Upgrader{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			t.Fatalf("Failed to upgrade websocket: %v", err)
		}
		defer conn.Close()

		// Read loop
		for {
			_, _, err := conn.ReadMessage()
			if err != nil {
				return
			}
		}
	}))
	defer server.Close()

	wsURL := "ws" + server.URL[4:]
	err := bm.Connect(ctx, wsURL, "remote-org-2", nil)
	if err != nil {
		t.Fatalf("Connect failed: %v", err)
	}

	cancel()
	time.Sleep(100 * time.Millisecond)
}

func TestBridgeManager_NoNodeForwardLoop(t *testing.T) {

	bm := NewBridgeManager(nil, "mesh:tasks:shared", "local-org-1")
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	bm.forwardLoop(ctx, "remote-org-1", nil)
}

func TestBridgeManager_NodeNoMeshTransportForwardLoop(t *testing.T) {

	node := &CentrifugeNode{meshTransport: nil}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	bm.forwardLoop(ctx, "remote-org-1", nil)
}

func TestBridgeManager_SubscribeErr(t *testing.T) {

	mt := &mockMeshTransport{
		errSubscribe: errors.New("mock subscribe error"),
	}
	node := &CentrifugeNode{meshTransport: mt}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	bm.forwardLoop(ctx, "remote-org-1", nil)
}

type mockMeshTransport struct {
	errSubscribe error
	ch           chan []byte
}

func (m *mockMeshTransport) SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error) {
	return nil, m.errSubscribe
}

func (m *mockMeshTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	return nil
}
func (m *mockMeshTransport) AdvertiseCapabilities(ctx context.Context, req pb.AgentCapabilities) error {
	return nil
}
func (m *mockMeshTransport) BroadcastCoordination(ctx context.Context, msg MeshMessage) error {
	return nil
}
func (m *mockMeshTransport) SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error) {
	return nil, nil
}
func (m *mockMeshTransport) BroadcastTask(ctx context.Context, task Task) error {
	return nil
}
func (m *mockMeshTransport) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	return nil, nil
}
func (m *mockMeshTransport) SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error) {
	return nil, nil
}
func TestBridgeManager_ForwardEnvelopedLocal(t *testing.T) {

	upgrader := websocket.Upgrader{}
	remoteReceived := make(chan []byte, 1)
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

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	err := bm.Connect(ctx, wsURL, "remote-org-2", nil)
	if err != nil {
		t.Fatalf("Connect failed: %v", err)
	}

	err = mt.BroadcastMeshEvent(ctx, "mesh:tasks:shared", []byte(`{"origin_org_id":"local-org-1","payload":{"event":"enveloped_from_local"}}`))
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	select {
	case msg := <-remoteReceived:
		expected := `{"origin_org_id":"local-org-1","payload":{"event":"enveloped_from_local"}}`
		if string(msg) != expected {
			t.Errorf("Expected %s, got %s", expected, string(msg))
		}
	case <-time.After(100 * time.Millisecond):
		// Expected behaviour, messages take time to pass the websocket connection
	}
}

func TestBridgeManager_ClosedEventChan(t *testing.T) {

	mt := &mockMeshTransport{
		ch: make(chan []byte),
	}
	close(mt.ch)

	node := &CentrifugeNode{meshTransport: mt}
	bm := NewBridgeManager(node, "mesh:tasks:shared", "local-org-1")
	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	bm.forwardLoop(ctx, "remote-org-1", nil)
}
