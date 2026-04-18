package orchestration

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestClaimTask_Unified(t *testing.T) {
    telemetry.InitTelemetry()
    dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to create sqlite provider: %v", err)
    }

    ctx := context.Background()

    _, err = dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_plan_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            dependencies TEXT,
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

    _, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status) VALUES ('task-1', 'org-1', 'Test Task 1', 'PENDING')")
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    claims := &auth.Claims{OrganizationID: "org-1"}
    ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

    to := NewSharedTaskOrchestrator(dbProvider, nil, nil)

    task, err := to.ClaimTask(ctxWithClaims, "spiffe://onehumancorp.io/agent/1")
    if err != nil {
        t.Fatalf("ClaimTask failed: %v", err)
    }

    if task == nil {
        t.Fatalf("expected to claim task-1, got nil")
    }

    if task.ID != "task-1" {
        t.Errorf("expected task ID 'task-1', got '%s'", task.ID)
    }

    if task.Status != "ASSIGNED" {
        t.Errorf("expected status 'ASSIGNED', got '%s'", task.Status)
    }
}
