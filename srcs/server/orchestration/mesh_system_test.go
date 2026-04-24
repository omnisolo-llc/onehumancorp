package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/alicebob/miniredis/v2"
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

    // Test Publish/Subscribe API directly
    ch2, err := mesh.Subscribe("test_topic")
    if err != nil {
        t.Fatalf("failed to subscribe: %v", err)
    }
    err = mesh.Publish("test_topic", []byte("data1"))
    if err != nil {
        t.Fatalf("failed to publish: %v", err)
    }
    select {
    case received := <-ch2:
        if string(received) != "data1" {
            t.Errorf("expected data1, got %s", received)
        }
    case <-time.After(2 * time.Second):
        t.Fatal("timeout waiting for broadcasted task")
    }

}

func TestRedisMeshTransport_PublishAndSubscribe(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer mr.Close()

	mesh, err := NewRedisMeshTransport(mr.Addr())
	if err != nil {
		t.Fatalf("failed to create redis mesh: %v", err)
	}

	ch, err := mesh.Subscribe("test_topic")
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	// Allow some time for redis pubsub subscription to connect
	time.Sleep(100 * time.Millisecond)

	err = mesh.Publish("test_topic", []byte("data1"))
	if err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	select {
	case received := <-ch:
		if string(received) != "data1" {
			t.Errorf("expected data1, got %s", received)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("timeout waiting for broadcasted task")
	}
}
