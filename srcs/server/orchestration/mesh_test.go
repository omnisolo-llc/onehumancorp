package orchestration

import (
	"context"
	"encoding/json"
	"testing"
	"time"
    "database/sql"

	"github.com/alicebob/miniredis/v2"
    _ "github.com/mattn/go-sqlite3"
)

func TestMeshPublishAndSubscribe(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to create miniredis: %v", err)
	}
	defer s.Close()

	mesh := NewTeammateMesh(s.Addr(), nil, false)
	ctx := context.Background()

	ch := make(chan MeshPayload, 1)
	mesh.SubscribeToTasks(ctx, func(p MeshPayload) {
		ch <- p
	})

    // Give subscription time to start
    time.Sleep(100 * time.Millisecond)

	err = mesh.PublishTaskStatus(ctx, "agent-1", "do_work", "PENDING", json.RawMessage(`{"key":"value"}`))
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

    // Give message time to be processed
    time.Sleep(100 * time.Millisecond)

    mesh.PublishTaskStatus(ctx, "agent-1", "do_work", "COMPLETED", nil)
    time.Sleep(100 * time.Millisecond)
}

func TestMeshPublishError(t *testing.T) {
	mesh := NewTeammateMesh("localhost:9999", nil, false) // Invalid port
	err := mesh.PublishTaskStatus(context.Background(), "agent-1", "do_work", "PENDING", nil)
	if err == nil {
		t.Error("Expected connection error")
	}
}

func TestMeshSubscribeError(t *testing.T) {
    s, _ := miniredis.Run()
    defer s.Close()
    mesh := NewTeammateMesh(s.Addr(), nil, false)

    mesh.SubscribeToTasks(context.Background(), func(p MeshPayload) {})

    // Publish invalid JSON to trigger unmarshal error
    err := mesh.rdb.Publish(context.Background(), "mesh:tasks", "{invalid JSON").Err()
    if err != nil {
        t.Fatalf("failed to publish invalid json: %v", err)
    }

    time.Sleep(100 * time.Millisecond)
}

func TestAutoDreamClient(t *testing.T) {
    db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open DB: %v", err)
	}
	defer db.Close()

    _, err = db.Exec(`CREATE TABLE autodream_memories (
        id TEXT PRIMARY KEY,
        organization_id TEXT,
        agent_id TEXT,
        task_id TEXT,
        content TEXT,
        embedding TEXT,
        source_type TEXT
    )`)
    if err != nil {
        t.Fatalf("Failed to create table: %v", err)
    }

	client := &AutoDreamClient{db: db, isSQLite: true}
	err = client.SummarizeTask(context.Background(), MeshPayload{AgentID: "agent-1", Action: "test"})
    if err != nil {
        t.Fatalf("Failed to summarize task sqlite: %v", err)
    }

    clientPg := &AutoDreamClient{db: db, isSQLite: false}
    err = clientPg.SummarizeTask(context.Background(), MeshPayload{AgentID: "agent-1", Action: "test"})
    if err == nil {
        t.Fatalf("Expected error due to postgres syntax in sqlite db")
    }

    clientNil := &AutoDreamClient{db: nil, isSQLite: false}
    err = clientNil.SummarizeTask(context.Background(), MeshPayload{AgentID: "agent-1", Action: "test"})
    if err != nil {
        t.Fatalf("Expected nil err for nil db: %v", err)
    }
}
