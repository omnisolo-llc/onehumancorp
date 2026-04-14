package orchestration

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestClaimTask_SQLite(t *testing.T) {
    telemetry.InitTelemetry()
    dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to create sqlite provider: %v", err)
    }

    ctx := context.Background()

    _, err = dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
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
        )
    `)
    if err != nil {
        t.Fatalf("failed to create shared_tasks: %v", err)
    }

    _, err = dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            reason TEXT,
            occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    `)
    if err != nil {
        t.Fatalf("failed to create state_machine_transitions: %v", err)
    }

    _, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_v4 (id, organization_id, title, status, dependencies) VALUES ('task-1', 'org-1', 'Test Task 1', 'COMPLETED', '[]')")
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    _, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_v4 (id, organization_id, title, status, dependencies) VALUES ('task-2', 'org-1', 'Test Task 2', 'PENDING', '[\"task-1\"]')")
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    // Dependency added directly above in JSON format

    _, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_v4 (id, organization_id, title, status, dependencies) VALUES ('task-3', 'org-1', 'Test Task 3', 'PENDING', '[\"task-2\"]')")
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    // Dependency added directly above in JSON format

    claims := &auth.Claims{OrganizationID: "org-1"}
    ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

    to := NewSharedTaskOrchestrator(dbProvider)

    task, err := to.ClaimTask(ctxWithClaims, "agent-1")
    if err != nil {
        t.Fatalf("ClaimTask failed: %v", err)
    }

    if task == nil {
        t.Fatalf("expected to claim task-2, got nil")
    }

    if task.ID != "task-2" {
        t.Errorf("expected task ID 'task-2', got '%s'", task.ID)
    }

    if task.Status != "IN_PROGRESS" {
        t.Errorf("expected status 'IN_PROGRESS', got '%s'", task.Status)
    }

    task3, err := to.ClaimTask(ctxWithClaims, "agent-2")
    if err != nil {
        t.Fatalf("ClaimTask failed: %v", err)
    }

    if task3 != nil {
        t.Fatalf("expected nil task for task-3, got %v", task3)
    }

    err = to.TransitionTask(ctxWithClaims, "task-2", "agent-1", "IN_PROGRESS", "COMPLETED", "Starting work")
    if err != nil {
        t.Fatalf("TransitionTask failed: %v", err)
    }
}

func TestClaimTask_Postgres(t *testing.T) {
    telemetry.InitTelemetry()
    dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to create sqlite provider: %v", err)
    }

    ctx := context.Background()

    _, err = dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
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
        )
    `)
    if err != nil {
        t.Fatalf("failed to create shared_tasks: %v", err)
    }

    _, err = dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            reason TEXT,
            occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    `)
    if err != nil {
        t.Fatalf("failed to create state_machine_transitions: %v", err)
    }

    _, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_v4 (id, organization_id, title, status, dependencies) VALUES ('task-1', 'org-1', 'Test Task 1', 'COMPLETED', '[]')")
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    _, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_v4 (id, organization_id, title, status, dependencies) VALUES ('task-2', 'org-1', 'Test Task 2', 'PENDING', '[\"task-1\"]')")
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    // Dependency added directly above in JSON format

    to := NewSharedTaskOrchestrator(dbProvider)

    task, err := to.claimTaskPostgres(ctx, "org-1", "agent-pg")
    if err != nil {
        t.Fatalf("claimTaskPostgres failed: %v", err)
    }

    if task == nil {
        t.Fatalf("expected to claim task-2, got nil")
    }

    if task.ID != "task-2" {
        t.Errorf("expected task ID 'task-2', got '%s'", task.ID)
    }

    if task.Status != "IN_PROGRESS" {
        t.Errorf("expected status 'IN_PROGRESS', got '%s'", task.Status)
    }
}
