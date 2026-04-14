package orchestration

import (
	"context"
	"testing"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	_ "modernc.org/sqlite"
)

func TestDecompositionTaskStore_SQLite(t *testing.T) {
	telemetry.InitTelemetry()
	sqldb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	dbProvider := db.NewSqliteProvider(sqldb)

	ctx := context.Background()

	_, err = dbProvider.Exec(ctx, `
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
        )
    `)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, dependencies) VALUES ('task-1', 'org-1', 'Test Task 1', 'DONE', '[]')")
	if err != nil {
		t.Fatalf("failed to insert task 1: %v", err)
	}

	_, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, dependencies) VALUES ('task-2', 'org-1', 'Test Task 2', 'PENDING', '[\"task-1\"]')")
	if err != nil {
		t.Fatalf("failed to insert task 2: %v", err)
	}

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	store := NewDecompositionTaskStore(dbProvider)

	task, err := store.ClaimTask(ctxWithClaims, "agent-1")
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
}

func TestDecompositionTaskStore_Postgres(t *testing.T) {
	telemetry.InitTelemetry()
	sqldb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	dbProvider := db.NewSqliteProvider(sqldb)

	ctx := context.Background()

	_, err = dbProvider.Exec(ctx, `
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
        )
    `)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, dependencies) VALUES ('task-1', 'org-1', 'Test Task 1', 'DONE', '[]')")
	if err != nil {
		t.Fatalf("failed to insert task 1: %v", err)
	}

	_, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, dependencies) VALUES ('task-2', 'org-1', 'Test Task 2', 'PENDING', '[\"task-1\"]')")
	if err != nil {
		t.Fatalf("failed to insert task 2: %v", err)
	}

	store := NewDecompositionTaskStore(dbProvider)

	if dbProvider.IsSQLite() {
		t.Skip("Skipping Postgres-specific test when using SQLite provider")
		return
	}

	task, err := store.claimTaskPostgres(ctx, "org-1", "agent-pg")
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
