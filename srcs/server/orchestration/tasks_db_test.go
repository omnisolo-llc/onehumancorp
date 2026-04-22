package orchestration

import (
    "context"
    "strings"
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

    if task.Status != "IN_PROGRESS" {
        t.Errorf("expected status 'IN_PROGRESS', got '%s'", task.Status)
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

    if task.Status != "IN_PROGRESS" {
        t.Errorf("expected status 'IN_PROGRESS', got '%s'", task.Status)
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

// mockPGProvider wraps a db.Provider to simulate Postgres
type mockPGProvider struct {
	db.Provider
}

func (m *mockPGProvider) IsSQLite() bool {
	return false
}

// Intercept queries with FOR UPDATE SKIP LOCKED
func (m *mockPGProvider) Begin(ctx context.Context) (db.Tx, error) {
	tx, err := m.Provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	return &mockPGTx{Tx: tx}, nil
}

type mockPGTx struct {
	db.Tx
}

func (m *mockPGTx) QueryRow(ctx context.Context, sql string, args ...interface{}) db.Row {
	// Strip FOR UPDATE SKIP LOCKED for SQLite backend
	if len(sql) > 22 && sql[len(sql)-22:] == "FOR UPDATE SKIP LOCKED" {
		sql = sql[:len(sql)-22]
	}
	// Also handle trailing spaces/newlines
	for len(sql) > 0 && (sql[len(sql)-1] == ' ' || sql[len(sql)-1] == '\n' || sql[len(sql)-1] == '\t' || sql[len(sql)-1] == '\r') {
		sql = sql[:len(sql)-1]
		if len(sql) > 22 && sql[len(sql)-22:] == "FOR UPDATE SKIP LOCKED" {
			sql = sql[:len(sql)-22]
		}
	}

	// A more robust replacement just in case
	sql = strings.ReplaceAll(sql, "FOR UPDATE SKIP LOCKED", "")
	return m.Tx.QueryRow(ctx, sql, args...)
}


func TestTasksDBClaimTask(t *testing.T) {
	_ = t.TempDir() // use TempDir as per guidelines
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create table
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies JSONB,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status)
		VALUES ('task-1', 'org-1', 'Test Task', 'PENDING')
	`)
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	tasksDB := NewTasksDB(provider)

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	task, err := tasksDB.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task == nil {
		t.Fatalf("ClaimTask returned nil task")
	}

	if task.TaskID != "task-1" {
		t.Errorf("Expected task ID task-1, got %s", task.TaskID)
	}

	if task.Status != "ASSIGNED" {
		t.Errorf("Expected task status ASSIGNED, got %s", task.Status)
	}

	if task.AgentID != "agent-1" {
		t.Errorf("Expected task agent ID agent-1, got %s", task.AgentID)
	}

	// Verify DB state
	var status, assignedAgentID string
	err = provider.QueryRow(ctx, "SELECT status, assigned_agent_id FROM shared_tasks WHERE id = 'task-1'").Scan(&status, &assignedAgentID)
	if err != nil {
		t.Fatalf("failed to query DB: %v", err)
	}

	if status != "ASSIGNED" {
		t.Errorf("Expected DB status ASSIGNED, got %s", status)
	}

	if assignedAgentID != "agent-1" {
		t.Errorf("Expected DB assigned agent ID agent-1, got %s", assignedAgentID)
	}
}

func TestTasksDBClaimTask_Postgres(t *testing.T) {
	_ = t.TempDir() // use TempDir as per guidelines
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Wrap provider to simulate Postgres
	pgProvider := &mockPGProvider{Provider: provider}

	// Create table
	_, err := pgProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies JSONB,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = pgProvider.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status)
		VALUES ('task-pg-1', 'org-pg', 'Test PG Task', 'PENDING')
	`)
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	tasksDB := NewTasksDB(pgProvider)

	claims := &auth.Claims{OrganizationID: "org-pg"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	task, err := tasksDB.ClaimTask(ctxWithClaims, "agent-pg")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task == nil {
		t.Fatalf("ClaimTask returned nil task")
	}

	if task.TaskID != "task-pg-1" {
		t.Errorf("Expected task ID task-pg-1, got %s", task.TaskID)
	}

	if task.Status != "ASSIGNED" {
		t.Errorf("Expected task status ASSIGNED, got %s", task.Status)
	}

	if task.AgentID != "agent-pg" {
		t.Errorf("Expected task agent ID agent-pg, got %s", task.AgentID)
	}

	// Verify DB state
	var status, assignedAgentID string
	err = pgProvider.QueryRow(ctx, "SELECT status, assigned_agent_id FROM shared_tasks WHERE id = 'task-pg-1'").Scan(&status, &assignedAgentID)
	if err != nil {
		t.Fatalf("failed to query DB: %v", err)
	}

	if status != "ASSIGNED" {
		t.Errorf("Expected DB status ASSIGNED, got %s", status)
	}

	if assignedAgentID != "agent-pg" {
		t.Errorf("Expected DB assigned agent ID agent-pg, got %s", assignedAgentID)
	}
}


func TestTasksDBClaimTask_NoPending(t *testing.T) {
	_ = t.TempDir()
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create table
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies JSONB,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	tasksDB := NewTasksDB(provider)

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	task, err := tasksDB.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task != nil {
		t.Fatalf("ClaimTask returned task when none was pending")
	}
}

func TestSharedTaskOrchestrator_ClaimTask(t *testing.T) {
	_ = t.TempDir()
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create table
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies JSONB,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            entity_id TEXT,
            entity_type TEXT,
            from_state TEXT,
            to_state TEXT,
            agent_id TEXT,
            reason TEXT,
            occurred_at TIMESTAMPTZ
        )
    `)
	if err != nil {
		t.Fatalf("Failed to create transitions table: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status, dependencies) VALUES ('task-1', 'org-1', 'Task 1', 'PENDING', '[]')")
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	orchestrator := NewSharedTaskOrchestrator(provider, nil, nil)

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	task, err := orchestrator.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task == nil {
		t.Fatalf("ClaimTask returned nil task")
	}

	if task.TaskID != "task-1" {
		t.Errorf("Expected task ID task-1, got %s", task.TaskID)
	}

	var status, assignedAgentID string
	err = provider.QueryRow(ctx, "SELECT status, assigned_agent_id FROM shared_tasks WHERE id = 'task-1'").Scan(&status, &assignedAgentID)
	if err != nil {
		t.Fatalf("Failed to fetch task from DB: %v", err)
	}

	if status != "IN_PROGRESS" {
		t.Errorf("Expected DB status IN_PROGRESS, got %s", status)
	}
	if assignedAgentID != "agent-1" {
		t.Errorf("Expected DB assigned agent ID agent-1, got %s", assignedAgentID)
	}
}

func TestSharedTaskOrchestrator_ClaimTask_Postgres(t *testing.T) {
	_ = t.TempDir()
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Wrap provider to simulate Postgres
	pgProvider := &mockPgProvider{Provider: provider}

	_, err := pgProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies JSONB,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	_, err = pgProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            entity_id TEXT,
            entity_type TEXT,
            from_state TEXT,
            to_state TEXT,
            agent_id TEXT,
            reason TEXT,
            occurred_at TIMESTAMPTZ
        )
    `)
	if err != nil {
		t.Fatalf("Failed to create transitions table: %v", err)
	}

	_, err = pgProvider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id, title, status, dependencies) VALUES ('task-pg-1', 'org-pg', 'Task PG 1', 'PENDING', '[]')")
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	orchestrator := NewSharedTaskOrchestrator(pgProvider, nil, nil)

	claims := &auth.Claims{OrganizationID: "org-pg"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	task, err := orchestrator.ClaimTask(ctxWithClaims, "agent-pg")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task == nil {
		t.Fatalf("ClaimTask returned nil task")
	}

	if task.TaskID != "task-pg-1" {
		t.Errorf("Expected task ID task-pg-1, got %s", task.TaskID)
	}

	var status, assignedAgentID string
	err = pgProvider.QueryRow(ctx, "SELECT status, assigned_agent_id FROM shared_tasks WHERE id = 'task-pg-1'").Scan(&status, &assignedAgentID)
	if err != nil {
		t.Fatalf("Failed to fetch task from DB: %v", err)
	}

	if status != "IN_PROGRESS" {
		t.Errorf("Expected DB status IN_PROGRESS, got %s", status)
	}
	if assignedAgentID != "agent-pg" {
		t.Errorf("Expected DB assigned agent ID agent-pg, got %s", assignedAgentID)
	}
}
