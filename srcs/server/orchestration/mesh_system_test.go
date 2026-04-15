package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestLocalTeammateMesh_BroadcastAndSubscribe(t *testing.T) {
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

	mesh := NewLocalTeammateMesh(provider)

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
