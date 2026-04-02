package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func resetSyncPrimitives() {
    // Optional helper function if we use sync Once in package.
}

func TestLocalTeammateMesh_StandaloneMode(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	mesh := NewLocalTeammateMesh(provider)

	ch, err := mesh.SubscribeTasks(ctx)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	task := Task{
		AgentID: "agent-123",
		Action:  "PICK_UP_TASK",
		Status:  "IN_PROGRESS",
		TaskID:  "task-456",
	}

	err = mesh.BroadcastTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to broadcast: %v", err)
	}

	select {
	case received := <-ch:
		if received.TaskID != "task-456" {
			t.Errorf("expected task-456, got %v", received.TaskID)
		}
		if received.AgentID != "agent-123" {
			t.Errorf("expected agent-123, got %v", received.AgentID)
		}
	case <-time.After(time.Second):
		t.Fatal("timed out waiting for task broadcast")
	}

	// Verify persistence
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", "task-456").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query db: %v", err)
	}
	if status != "IN_PROGRESS" {
		t.Errorf("expected status IN_PROGRESS, got %s", status)
	}
}

func TestMeshManager_StandaloneMode(t *testing.T) {
	mesh, err := NewMeshManager("")
	if err != nil {
		t.Fatalf("failed to create mesh: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		mesh.HandleWebSocket(w, r, "room-1")
	}))
	defer server.Close()

	url := "ws" + strings.TrimPrefix(server.URL, "http")
	conn, _, err := websocket.DefaultDialer.Dial(url, nil)
	if err != nil {
		t.Fatalf("failed to dial: %v", err)
	}
	defer conn.Close()

	go func() {
		// Wait a bit for subscription
		time.Sleep(100 * time.Millisecond)
		_ = mesh.Publish(context.Background(), "room-1", `{"content":"hello"}`)
	}()

	_, msg, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("read message failed: %v", err)
	}

	if !strings.Contains(string(msg), "hello") {
		t.Errorf("expected hello, got %s", string(msg))
	}
}

func TestMeshManager_Publish(t *testing.T) {
	mesh, err := NewMeshManager("")
	if err != nil {
		t.Fatalf("failed to create mesh: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		mesh.HandleWebSocket(w, r, "room-1")
	}))
	defer server.Close()

	url := "ws" + strings.TrimPrefix(server.URL, "http")
	conn1, _, err := websocket.DefaultDialer.Dial(url, nil)
	if err != nil {
		t.Fatalf("failed to dial conn1: %v", err)
	}
	defer conn1.Close()

	conn2, _, err := websocket.DefaultDialer.Dial(url, nil)
	if err != nil {
		t.Fatalf("failed to dial conn2: %v", err)
	}
	defer conn2.Close()

	time.Sleep(100 * time.Millisecond) // wait for subscriptions

	err = mesh.Publish(context.Background(), "room-1", `{"content":"broadcast"}`)
	if err != nil {
		t.Fatalf("publish failed: %v", err)
	}

	for _, conn := range []*websocket.Conn{conn1, conn2} {
		_, msg, err := conn.ReadMessage()
		if err != nil {
			t.Fatalf("read failed: %v", err)
		}
		if !strings.Contains(string(msg), "broadcast") {
			t.Errorf("expected broadcast, got %s", string(msg))
		}
	}
}

func TestMeshManager_MultiTenantIsolation(t *testing.T) {
	// Verify that meshes do not leak across tenants by isolating room channels.

	mesh, err := NewMeshManager("")
	if err != nil {
		t.Fatalf("failed to create mesh: %v", err)
	}

	server1 := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		mesh.HandleWebSocket(w, r, "room-tenant-a")
	}))
	defer server1.Close()

	server2 := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		mesh.HandleWebSocket(w, r, "room-tenant-b")
	}))
	defer server2.Close()

	conn1, _, err := websocket.DefaultDialer.Dial("ws"+strings.TrimPrefix(server1.URL, "http"), nil)
	if err != nil {
		t.Fatalf("dial tenant a failed: %v", err)
	}
	defer conn1.Close()

	conn2, _, err := websocket.DefaultDialer.Dial("ws"+strings.TrimPrefix(server2.URL, "http"), nil)
	if err != nil {
		t.Fatalf("dial tenant b failed: %v", err)
	}
	defer conn2.Close()

	time.Sleep(100 * time.Millisecond)

	// Send to Tenant A
	err = conn1.WriteMessage(websocket.TextMessage, []byte(`{"content": "ping A"}`))
	if err != nil {
		t.Fatalf("write failed: %v", err)
	}

	// Receive on Tenant A
	_, msg, err := conn1.ReadMessage()
	if err != nil {
		t.Fatalf("read failed: %v", err)
	}
	if !strings.Contains(string(msg), "ping A") {
		t.Errorf("expected ping A, got %s", string(msg))
	}

	// Ensure Tenant B doesn't receive it
	conn2.SetReadDeadline(time.Now().Add(50 * time.Millisecond))
	_, _, err = conn2.ReadMessage()
	if err == nil {
		t.Fatal("tenant B should not have received message from A")
	}
}
