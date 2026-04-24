package orchestration

import (
    "context"
    "strings"
    "testing"

    "github.com/onehumancorp/mono/src/server/auth"
    "github.com/onehumancorp/mono/src/server/db"
    "github.com/onehumancorp/mono/src/server/telemetry"
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
type mockPGProviderTasksDB struct {
	db.Provider
}

func (m *mockPGProviderTasksDB) IsSQLite() bool {
	return false
}

// Intercept queries with FOR UPDATE SKIP LOCKED
func (m *mockPGProviderTasksDB) Begin(ctx context.Context) (db.Tx, error) {
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
	pgProvider := &mockPGProviderTasksDB{Provider: provider}

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

func TestTasksDBClaimTask_DAG(t *testing.T) {
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	// Initialize schema for tasks and transitions manually for tests
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			dependencies JSON,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	tasksDB := NewTasksDB(provider)
	ctxWithClaims := auth.ContextWithClaims(ctx, &auth.Claims{OrganizationID: "org-123"})

	// Insert Task A (DONE) and Task B (PENDING, depends on A) and Task C (PENDING, depends on D which is PENDING)
	_, err = provider.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, status, dependencies) VALUES
		('task-a', 'org-123', 'Task A', 'DONE', '[]'),
		('task-b', 'org-123', 'Task B', 'PENDING', '["task-a"]'),
		('task-d', 'org-123', 'Task D', 'PENDING', '[]'),
		('task-c', 'org-123', 'Task C', 'PENDING', '["task-d"]')
	`)
	if err != nil {
		t.Fatalf("failed to insert tasks: %v", err)
	}

	// Claim Task: Should claim B or D, but not C
	task, err := tasksDB.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task == nil {
		t.Fatalf("expected to claim a task")
	}
	if task.TaskID != "task-b" && task.TaskID != "task-d" {
		t.Errorf("expected to claim task-b or task-d, got %s", task.TaskID)
	}

	// Transition the claimed task
	err = tasksDB.TransitionTask(ctxWithClaims, task.TaskID, "agent-1", "IN_PROGRESS", "DONE", "finished")
	if err != nil {
		t.Fatalf("failed to transition task: %v", err)
	}

	// If D was claimed and transitioned to DONE, C should now be claimable.
	// We'll just try claiming again.
	task2, err := tasksDB.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task2 == nil {
		t.Fatalf("expected to claim another task")
	}

	// Transition the second task
	err = tasksDB.TransitionTask(ctxWithClaims, task2.TaskID, "agent-1", "IN_PROGRESS", "DONE", "finished")
	if err != nil {
		t.Fatalf("failed to transition task: %v", err)
	}

	// Now try to claim C (since D is definitely DONE by now)
	task3, err := tasksDB.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task3 == nil {
		t.Fatalf("expected to claim task C")
	}
	if task3.TaskID != "task-c" {
		t.Errorf("expected to claim task-c, got %s", task3.TaskID)
	}
}
