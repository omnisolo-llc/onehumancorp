package orchestration

import (
    "context"
    "database/sql"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func newTestProvider(t *testing.T) db.Provider {
    t.Helper()
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open test sqlite db: %v", err)
    }

    if err := sqliteDB.PingContext(context.Background()); err != nil {
        t.Fatalf("failed to ping test sqlite db: %v", err)
    }

    t.Cleanup(func() {
        sqliteDB.Close()
    })

    return db.NewSqliteProvider(sqliteDB)
}

func TestKairosTaskOrchestrator_ClaimTask(t *testing.T) {
    provider := newTestProvider(t)
    orchestrator := NewKairosTaskOrchestrator(provider)

    // Ensure tables exist for SQLite fallback testing.
    // Note that in a real environment database.go translates UUID PRIMARY KEY DEFAULT gen_random_uuid() to TEXT PRIMARY KEY.
    _, err := provider.Exec(context.Background(), `
    CREATE TABLE IF NOT EXISTS shared_tasks_kairos (
        id TEXT PRIMARY KEY,
        organization_id TEXT NOT NULL,
        title TEXT NOT NULL,
        description TEXT,
        status TEXT NOT NULL DEFAULT 'PENDING',
        agent_id TEXT,
        priority TEXT NOT NULL DEFAULT 'P2',
        payload TEXT,
        parent_plan_id TEXT,
        dependencies TEXT NOT NULL DEFAULT '[]',
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )`)
    if err != nil {
        t.Fatalf("failed to setup schema: %v", err)
    }

    orgID := "org-1"
    _, err = provider.Exec(context.Background(), "INSERT INTO shared_tasks_kairos (id, organization_id, title) VALUES ('task-1', $1, 'Test Task')", orgID)
    if err != nil {
        t.Fatalf("failed to insert test data: %v", err)
    }

    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: orgID})
    task, err := orchestrator.ClaimTask(ctx, "agent-1")
    if err != nil {
        t.Fatalf("ClaimTask failed: %v", err)
    }

    if task == nil {
        t.Fatalf("Expected task to be returned, got nil")
    }

    if task.ID != "task-1" {
        t.Errorf("Expected task ID 'task-1', got '%s'", task.ID)
    }

    if task.Status != "IN_PROGRESS" {
        t.Errorf("Expected status 'IN_PROGRESS', got '%s'", task.Status)
    }

    if *task.AgentID != "agent-1" {
        t.Errorf("Expected agent_id 'agent-1', got '%s'", *task.AgentID)
    }

    // Verify the database state was actually updated
    var dbStatus string
    provider.QueryRow(context.Background(), "SELECT status FROM shared_tasks_kairos WHERE id = 'task-1'").Scan(&dbStatus)
    if dbStatus != "IN_PROGRESS" {
        t.Errorf("Expected db status 'IN_PROGRESS', got '%s'", dbStatus)
    }
}
