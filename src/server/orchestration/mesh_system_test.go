package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

func TestMemoryMeshTransport_BroadcastAndSubscribe(t *testing.T) {
	provider := db.NewTestProvider(t)
	_, err := provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			title TEXT,
			status TEXT,
			agent_id TEXT,
			organization_id TEXT,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	mesh := NewMemoryMeshTransport(provider)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	ch, err := mesh.SubscribeTasks(ctx)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	task := Task{
		AgentID: "spiffe://onehumancorp.io/agent/1",
		Action:  "CREATE",
		Status:  "PENDING",
		TaskID:  "task-123",
	}

	if err := mesh.BroadcastTask(ctx, task); err != nil {
		t.Fatalf("failed to broadcast: %v", err)
	}

	select {
	case received := <-ch:
		if received.TaskID != task.TaskID {
			t.Errorf("expected task ID %s, got %s", task.TaskID, received.TaskID)
		}
		if received.AgentID != task.AgentID {
			t.Errorf("expected agent ID %s, got %s", task.AgentID, received.AgentID)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("timeout waiting for broadcasted task")
	}
}

func TestMemoryMeshTransport_SubscribeMeshEventsWithFilter(t *testing.T) {
	provider := setupTestDB(t)
	mesh := NewMemoryMeshTransport(provider)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	topic := "filtered-topic"

	// Create filter to only accept messages containing "accept"
	filter := func(b []byte) bool {
		return string(b) == "accept"
	}

	sub, err := mesh.SubscribeMeshEventsWithFilter(ctx, topic, filter)
	if err != nil {
		t.Fatalf("SubscribeMeshEventsWithFilter failed: %v", err)
	}

	// Wait briefly for subscription to register
	time.Sleep(50 * time.Millisecond)

	if err := mesh.BroadcastMeshEvent(ctx, topic, []byte("reject")); err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	if err := mesh.BroadcastMeshEvent(ctx, topic, []byte("accept")); err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	// Should only receive "accept"
	select {
	case msg := <-sub:
		if string(msg) != "accept" {
			t.Errorf("Expected 'accept', got '%s'", string(msg))
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for filtered event")
	}

	// Ensure no more messages
	select {
	case msg := <-sub:
		t.Errorf("Received unexpected message: %s", string(msg))
	case <-time.After(50 * time.Millisecond):
		// Success
	}
}
