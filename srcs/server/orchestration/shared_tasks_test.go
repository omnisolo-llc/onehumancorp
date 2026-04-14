package orchestration

import (
	"context"
	"testing"
    "encoding/json"
    "database/sql"
    _ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDBSharedTasks(t *testing.T) db.Provider {
    conn, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }

    p := db.NewSqliteProvider(conn)

    ctx := context.Background()

    // Create the required table
    _, err = p.Exec(ctx, `
CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload TEXT,
    parent_plan_id TEXT,
    dependencies TEXT NOT NULL DEFAULT '[]',
    locked_until TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);`)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    return p
}

func TestClaimTask(t *testing.T) {
	provider := setupTestDBSharedTasks(t)
    defer provider.Close()

	ctx := context.Background()

    task := &SharedTaskDecomposition{
        OrganizationID: "org-1",
        Title: "Test Task",
        Status: "PENDING",
        Priority: "P2",
        Payload: json.RawMessage("{}"),
        Dependencies: json.RawMessage("[]"),
    }

    err := CreateTask(ctx, provider, task)
    if err != nil {
        t.Fatalf("failed to create task: %v", err)
    }

    claimedTask, err := ClaimTask(ctx, provider, "agent-1")
    if err != nil {
        t.Fatalf("failed to claim task: %v", err)
    }

    if claimedTask.AgentID.String != "agent-1" {
        t.Errorf("expected agent-1, got %v", claimedTask.AgentID.String)
    }

    if claimedTask.Status != "ASSIGNED" {
        t.Errorf("expected status ASSIGNED, got %v", claimedTask.Status)
    }

    err = TransitionTask(ctx, provider, claimedTask.ID, "DONE")
    if err != nil {
        t.Fatalf("failed to transition task: %v", err)
    }

    var status string
    err = provider.QueryRow(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = $1", claimedTask.ID).Scan(&status)
    if err != nil {
        t.Fatalf("failed to query status: %v", err)
    }
    if status != "DONE" {
        t.Errorf("expected status DONE, got %v", status)
    }
}
