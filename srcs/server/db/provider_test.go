package db

import (
	"context"
	"testing"
)

func TestConvertBindVarsJSONPath(t *testing.T) {
	query := "SELECT id FROM test WHERE payload::json->>'role' = $1 AND meta :: json ->> 'status' = $2"
	expected := "SELECT id FROM test WHERE json_extract(payload, '$.role') = ?1 AND json_extract(meta, '$.status') = ?2"

	result := convertBindVars(query)
	if result != expected {
		t.Errorf("convertBindVars() = %v, want %v", result, expected)
	}
}

func TestSqliteProviderIsSQLite(t *testing.T) {
	// Let's create an empty SqliteProvider and test its IsSQLite method.
	p := &SqliteProvider{}
	if !p.IsSQLite() {
		t.Errorf("SqliteProvider.IsSQLite() = %v, want true", p.IsSQLite())
	}
}

func TestPgProviderIsSQLite(t *testing.T) {
	// Let's create an empty PgProvider and test its IsSQLite method.
	p := &PgProvider{}
	if p.IsSQLite() {
		t.Errorf("PgProvider.IsSQLite() = %v, want false", p.IsSQLite())
	}
}

func TestStandaloneFallback(t *testing.T) {
	// Use an in-memory SQLite database to avoid creating files on disk.
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory&cache=shared")

	db, err := New(context.Background())
	if err != nil {
		t.Fatalf("Failed to initialize standalone db: %v", err)
	}
	defer db.Close()

	if !db.Provider.IsSQLite() {
		t.Errorf("Expected SQLite provider")
	}
}

func TestProvider_AcquireTask(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory&cache=shared")

	dbp, err := New(context.Background())
	if err != nil {
		t.Fatalf("Failed to initialize standalone db: %v", err)
	}
	defer dbp.Close()

	provider := dbp.Provider
	ctx := context.Background()

	// Setup SQLite table schema specific to our test
	schema := `
	CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		assigned_agent_id TEXT,
		priority TEXT NOT NULL DEFAULT 'P2',
		payload TEXT,
		parent_plan_id TEXT,
		dependencies TEXT NOT NULL DEFAULT '[]',
		locked_until DATETIME,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);`

	_, err = provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("Failed to create schema: %v", err)
	}

	// Insert test data
	insertQuery := `
		INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, created_at, updated_at)
		VALUES ('task-1', 'org-1', 'Task 1', 'PENDING', '2026-04-15 10:00:00', '2026-04-15 10:00:00'),
		       ('task-2', 'org-1', 'Task 2', 'PENDING', '2026-04-15 10:01:00', '2026-04-15 10:01:00');
	`
	_, err = provider.Exec(ctx, insertQuery)
	if err != nil {
		t.Fatalf("Failed to insert test tasks: %v", err)
	}

	// Create repository
	repo := NewSharedTaskRepository(provider)

	// Acquire a task
	task, err := repo.AcquireTask(ctx, "org-1", "agent-x")
	if err != nil {
		t.Fatalf("Failed to acquire task: %v", err)
	}
	if task == nil {
		t.Fatalf("Expected a task, got nil")
	}

	if task.ID != "task-1" {
		t.Errorf("Expected 'task-1' to be acquired due to earlier created_at, got '%s'", task.ID)
	}
	if task.Status != "IN_PROGRESS" {
		t.Errorf("Expected status IN_PROGRESS, got '%s'", task.Status)
	}
	if task.AgentID == nil || *task.AgentID != "agent-x" {
		t.Errorf("Expected agent_id 'agent-x', got %v", task.AgentID)
	}

	// Acquire next task
	task2, err := repo.AcquireTask(ctx, "org-1", "agent-y")
	if err != nil {
		t.Fatalf("Failed to acquire task 2: %v", err)
	}
	if task2 == nil {
		t.Fatalf("Expected a task 2, got nil")
	}
	if task2.ID != "task-2" {
		t.Errorf("Expected 'task-2', got '%s'", task2.ID)
	}

	// No more tasks
	task3, err := repo.AcquireTask(ctx, "org-1", "agent-z")
	if err != nil {
		t.Fatalf("Failed to acquire task 3: %v", err)
	}
	if task3 != nil {
		t.Errorf("Expected nil task, got %v", task3.ID)
	}
}

func TestKairosTasksSchema(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory&cache=shared")

	dbp, err := New(context.Background())
	if err != nil {
		t.Fatalf("Failed to initialize standalone db: %v", err)
	}
	defer dbp.Close()

	if err := dbp.RunMigrations(context.Background()); err != nil {
		t.Fatalf("Failed to run migrations: %v", err)
	}

	// Verify missions table
	_, err = dbp.Exec(context.Background(), "INSERT INTO missions (id, title, description, priority, status) VALUES ('123e4567-e89b-12d3-a456-426614174000', 'Test Mission', 'A test mission', 'P0', 'PENDING')")
	if err != nil {
		t.Fatalf("Failed to insert into missions table: %v", err)
	}

	var title string
	err = dbp.QueryRow(context.Background(), "SELECT title FROM missions WHERE id = '123e4567-e89b-12d3-a456-426614174000'").Scan(&title)
	if err != nil {
		t.Fatalf("Failed to query missions table: %v", err)
	}
	if title != "Test Mission" {
		t.Errorf("Expected title 'Test Mission', got '%s'", title)
	}

	// Verify agent_state table
	_, err = dbp.Exec(context.Background(), "INSERT INTO agent_state (agent_id, current_mission_id, status) VALUES ('agent-1', '123e4567-e89b-12d3-a456-426614174000', 'WORKING')")
	if err != nil {
		t.Fatalf("Failed to insert into agent_state table: %v", err)
	}

	var status string
	err = dbp.QueryRow(context.Background(), "SELECT status FROM agent_state WHERE agent_id = 'agent-1'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query agent_state table: %v", err)
	}
	if status != "WORKING" {
		t.Errorf("Expected status 'WORKING', got '%s'", status)
	}
}
