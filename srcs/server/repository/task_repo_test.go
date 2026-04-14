package repository

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	_ "modernc.org/sqlite"
)

func TestClaimTask_SQLite(t *testing.T) {
	telemetry.InitTelemetry()
	mockDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer mockDB.Close()
	dbProvider := db.NewSqliteProvider(mockDB)

	ctx := context.Background()

	_, err = dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
            id VARCHAR PRIMARY KEY,
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            assigned_agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            locked_until TIMESTAMP WITH TIME ZONE,
            payload TEXT,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
    `)
	if err != nil {
		t.Fatalf("failed to create shared_tasks_v4: %v", err)
	}

	_, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_v4 (id, organization_id, title, status) VALUES ('task-1', 'org-1', 'Test Task 1', 'ASSIGNED')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	_, err = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_v4 (id, organization_id, title, status) VALUES ('task-2', 'org-1', 'Test Task 2', 'PENDING')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	repo := NewTaskRepository(dbProvider)

	task, err := repo.ClaimTask(ctxWithClaims, "worker-1")
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

	task3, err := repo.ClaimTask(ctxWithClaims, "worker-2")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}

	if task3 != nil {
		t.Fatalf("expected nil task for task-3, got %v", task3)
	}
}
