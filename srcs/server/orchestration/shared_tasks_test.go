package orchestration

import (
	"context"
	"testing"
    "encoding/json"
    "database/sql"
    "strings"
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

    if *claimedTask.AssignedAgentID != "agent-1" {
        t.Errorf("expected agent-1, got %v", *claimedTask.AssignedAgentID)
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

    // Test claiming when no tasks are available
    noTask, err := ClaimTask(ctx, provider, "agent-1")
    if err != nil {
        t.Fatalf("expected nil error when no tasks are available, got %v", err)
    }
    if noTask != nil {
        t.Errorf("expected nil task when no tasks are available, got %v", noTask)
    }
}

type mockPGProvider struct {
	db.Provider
	lastQuery string
}

func (m *mockPGProvider) IsSQLite() bool { return false }
func (m *mockPGProvider) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
	m.lastQuery = query
	return &mockPGRow{}
}

type mockPGRow struct{}
func (m *mockPGRow) Scan(dest ...interface{}) error { return sql.ErrNoRows }

func TestClaimTask_PostgresLocking(t *testing.T) {
	ctx := context.Background()
	provider := &mockPGProvider{}

	_, err := ClaimTask(ctx, provider, "agent-1")
	if err != nil {
		t.Fatalf("expected nil error on sql.ErrNoRows, got %v", err)
	}

	if !strings.Contains(provider.lastQuery, "FOR UPDATE SKIP LOCKED") {
		t.Errorf("expected query to contain FOR UPDATE SKIP LOCKED, got: %s", provider.lastQuery)
	}
}


func TestClaimTask_DAGDependencies(t *testing.T) {
	provider := setupTestDBSharedTasks(t)
    defer provider.Close()

	ctx := context.Background()

    task1 := &SharedTaskDecomposition{
        OrganizationID: "org-1",
        Title: "Task 1",
        Status: "PENDING",
        Priority: "P2",
        Payload: json.RawMessage("{}"),
        Dependencies: json.RawMessage("[]"),
    }

    err := CreateTask(ctx, provider, task1)
    if err != nil {
        t.Fatalf("failed to create task1: %v", err)
    }

    task2 := &SharedTaskDecomposition{
        OrganizationID: "org-1",
        Title: "Task 2",
        Status: "PENDING",
        Priority: "P2",
        Payload: json.RawMessage("{}"),
        Dependencies: json.RawMessage("[\"" + task1.ID + "\"]"),
    }

    err = CreateTask(ctx, provider, task2)
    if err != nil {
        t.Fatalf("failed to create task2: %v", err)
    }

    // task2 should not be claimed because task1 is not completed
    claimed, err := ClaimTask(ctx, provider, "agent-1")
    if err != nil {
        t.Fatalf("failed to claim task: %v", err)
    }
    if claimed == nil {
        t.Fatalf("expected to claim task1, got nil")
    }
    if claimed.ID != task1.ID {
        t.Fatalf("expected to claim task1, got %v", claimed.ID)
    }

    // task2 is still blocked
    blocked, err := ClaimTask(ctx, provider, "agent-2")
    if err != nil {
        t.Fatalf("failed to claim task: %v", err)
    }
    if blocked != nil {
        t.Fatalf("expected task2 to be blocked, got %v", blocked)
    }

    // Complete task1
    err = TransitionTask(ctx, provider, task1.ID, "COMPLETED")
    if err != nil {
        t.Fatalf("failed to transition task1: %v", err)
    }

    // Now task2 should be claimable
    claimed2, err := ClaimTask(ctx, provider, "agent-2")
    if err != nil {
        t.Fatalf("failed to claim task: %v", err)
    }
    if claimed2 == nil {
        t.Fatalf("expected to claim task2, got nil")
    }
    if claimed2.ID != task2.ID {
        t.Fatalf("expected to claim task2, got %v", claimed2.ID)
    }
}
