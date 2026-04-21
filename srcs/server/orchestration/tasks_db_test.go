package orchestration

import (
    "context"
    "testing"
    "sync"
    "time"

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

    to := NewSharedTaskOrchestrator(dbProvider, nil, nil)

    task, err := to.ClaimTaskV4(ctxWithClaims, "org-1", "spiffe://onehumancorp.io/agent/1")
    if err != nil {
        t.Fatalf("ClaimTask failed: %v", err)
    }

    if task == nil {
        t.Fatalf("expected to claim task-2, got nil")
    }

    if task.ID != "task-2" {
        t.Errorf("expected task ID 'task-2', got '%s'", task.ID)
    }

    if task.Status != "ASSIGNED" {
        t.Errorf("expected status 'ASSIGNED', got '%s'", task.Status)
    }

    task3, err := to.ClaimTaskV4(ctxWithClaims, "org-1", "agent-2")
    if err != nil {
        t.Fatalf("ClaimTask failed: %v", err)
    }

    if task3 != nil {
        t.Fatalf("expected nil task for task-3, got %v", task3)
    }

    err = to.TransitionTask(ctxWithClaims, "task-2", "spiffe://onehumancorp.io/agent/1", "IN_PROGRESS", "COMPLETED", "Starting work")
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

    to := NewSharedTaskOrchestrator(dbProvider, nil, nil)

    task, err := to.ClaimTaskV4(ctx, "org-1", "agent-pg")
    if err != nil {
        t.Fatalf("ClaimTaskV4 failed: %v", err)
    }

    if task == nil {
        t.Fatalf("expected to claim task-2, got nil")
    }

    if task.ID != "task-2" {
        t.Errorf("expected task ID 'task-2', got '%s'", task.ID)
    }

    if task.Status != "ASSIGNED" {
        t.Errorf("expected status 'ASSIGNED', got '%s'", task.Status)
    }
}

func TestClaimTaskV4LockingPg(t *testing.T) {
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
            status TEXT NOT NULL DEFAULT 'PENDING',
            dependencies TEXT NOT NULL DEFAULT '[]'
        )
    `)
    if err != nil {
        t.Fatalf("failed to create shared_tasks_v4: %v", err)
    }

    _, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_v4 (id, organization_id, title, status, dependencies) VALUES ('task-lock-1', 'org-1', 'Test Lock 1', 'PENDING', '[]')")
    if err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    to := NewSharedTaskOrchestrator(dbProvider, nil, nil)

    // Test that the method successfully claims a task using the sqlite fallback logic
    task, err := to.ClaimTaskV4(ctx, "org-1", "agent-pg")
    if err != nil {
        t.Fatalf("ClaimTaskV4 should not have failed: %v", err)
    }
    if task == nil || task.ID != "task-lock-1" {
        t.Fatalf("Expected to claim task-lock-1")
    }
}

func TestClaimTaskDag(t *testing.T) {
    telemetry.InitTelemetry()
    dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to create sqlite provider: %v", err)
    }

    ctx := context.Background()
    claims := &auth.Claims{OrganizationID: "org-1"}
    ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

    _, err = dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]'
        )
    `)
    if err != nil { t.Fatalf("failed to create: %v", err) }

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
    if err != nil { t.Fatalf("failed to create state_machine_transitions: %v", err) }

    dbProvider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status, dependencies) VALUES ('task-1', 'org-1', 'Test 1', 'COMPLETED', '[]')")
    dbProvider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status, dependencies) VALUES ('task-2', 'org-1', 'Test 2', 'PENDING', '[\"task-1\"]')")

    to := NewSharedTaskOrchestrator(dbProvider, nil, nil)
    task, err := to.ClaimTask(ctxWithClaims, "agent-1")
    if err != nil {
        t.Fatalf("ClaimTask failed: %v", err)
    }
    if task == nil || task.TaskID != "task-2" {
        t.Fatalf("expected to claim task-2")
    }
}

type mockMesh struct {
    Events []string
    mu     sync.Mutex
}

func (m *mockMesh) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
    m.mu.Lock()
    defer m.mu.Unlock()
    m.Events = append(m.Events, topic)
    return nil
}

func TestClaimTaskNewSchema(t *testing.T) {
    telemetry.InitTelemetry()
    dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to create sqlite provider: %v", err)
    }

    ctx := context.Background()
    claims := &auth.Claims{OrganizationID: "org-1"}
    ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

    _, err = dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            parent_plan_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    `)
    if err != nil { t.Fatalf("failed to create shared_tasks: %v", err) }

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
    if err != nil { t.Fatalf("failed to create state_machine_transitions: %v", err) }

    _, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status, dependencies) VALUES ('task-new-1', 'org-1', 'Test New 1', 'COMPLETED', '[]')")
    if err != nil { t.Fatalf("failed to insert task: %v", err) }

    _, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status, dependencies) VALUES ('task-new-2', 'org-1', 'Test New 2', 'PENDING', '[\"task-new-1\"]')")
    if err != nil { t.Fatalf("failed to insert task: %v", err) }

    mesh := &mockMesh{}
    to := NewSharedTaskOrchestrator(dbProvider, mesh, nil)
    task, err := to.ClaimTask(ctxWithClaims, "agent-1")
    if err != nil {
        t.Fatalf("ClaimTask failed: %v", err)
    }
    if task == nil || task.TaskID != "task-new-2" {
        t.Fatalf("expected to claim task-new-2")
    }

    if task.Status != "ASSIGNED" {
        t.Errorf("expected status 'ASSIGNED', got '%s'", task.Status)
    }

    time.Sleep(10 * time.Millisecond) // Wait for async event
    mesh.mu.Lock()
    defer mesh.mu.Unlock()
    if len(mesh.Events) == 0 || mesh.Events[0] != "task.assigned" {
        t.Errorf("expected 'task.assigned' event, got %v", mesh.Events)
    }
}
