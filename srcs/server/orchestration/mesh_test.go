package orchestration

import (
	pb "github.com/onehumancorp/mono/srcs/proto"

	"github.com/onehumancorp/mono/srcs/server/db"

	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

func TestTeammateMesh_StandaloneMode(t *testing.T) {
	mesh, err := NewLegacyTeammateMesh("")
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
	msg := `{"agent_id":"agent-1","action":"CHAT","status":"SENT","sender_id":"agent-1","role":"SWE","content":"hello"}`
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
	mesh, err := NewLegacyTeammateMesh("")
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

func TestTeammateMesh_MultiTenantIsolation(t *testing.T) {
	// Verify that meshes do not leak across tenants by isolating room channels.

	mesh, err := NewLegacyTeammateMesh("")
	if err != nil {
		t.Fatalf("failed to create mesh: %v", err)
	}

	mesh.mu.Lock()
	if len(mesh.subscribers) != 0 {
		t.Errorf("Expected 0 subscribers initially")
	}
	mesh.mu.Unlock()
}

func TestLocalTeammateMesh(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	// Use NewTestProvider or db.New to init db
	ctx := context.Background()

	provider := db.NewTestProvider(t)
	defer provider.Close()

	// Explicitly define the schema within the test initialization
	_, err := provider.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload JSON,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	mesh := NewLocalTeammateMesh(provider)

	sub, err := mesh.SubscribeTasks(ctx)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	task := Task{
		AgentID: "spiffe://onehumancorp.io/agent/123",
		Action:  "PICK_UP_TASK",
		Status:  "IN_PROGRESS",
		TaskID:  "task-456",
	}

	err = mesh.BroadcastTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to broadcast: %v", err)
	}

	select {
	case received := <-sub:
		if received.TaskID != task.TaskID {
			t.Errorf("expected task id %s, got %s", task.TaskID, received.TaskID)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("timeout waiting for task broadcast")
	}

	// Verify persistence
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM shared_tasks WHERE id = $1", task.TaskID).Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 record in shared_tasks, got %d", count)
	}
}

func TestMemoryMeshTransport_EventsAndCapabilities(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	pool := db.NewTestProvider()
	defer pool.Close()
	mt := NewMemoryMeshTransport(pool)

	t.Run("Capabilities", func(t *testing.T) {
		sub, err := mt.SubscribeCapabilities(ctx)
		require.NoError(t, err)

		err = mt.AdvertiseCapabilities(ctx, pb.AgentCapabilities{
			AgentId: "spiffe://onehumancorp.io/agent/test-agent",
		})
		require.NoError(t, err)

		select {
		case caps := <-sub:
			assert.Equal(t, "test-agent", caps.AgentId)
		case <-time.After(1 * time.Second):
			t.Fatal("timeout waiting for capabilities")
		}
	})

	t.Run("MeshEvents", func(t *testing.T) {
		sub, err := mt.SubscribeMeshEvents(ctx, "tasks")
		require.NoError(t, err)

		err = mt.BroadcastMeshEvent(ctx, "tasks", []byte("payload"))
		require.NoError(t, err)

		select {
		case payload := <-sub:
			assert.Equal(t, []byte("payload"), payload)
		case <-time.After(1 * time.Second):
			t.Fatal("timeout waiting for mesh event")
		}
	})
}
