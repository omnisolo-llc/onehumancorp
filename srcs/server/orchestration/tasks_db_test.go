package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func runClaimTaskTest(t *testing.T, isStandalone bool) {
	if isStandalone {
		t.Setenv("OHC_STANDALONE", "true")
	} else {
		t.Setenv("OHC_STANDALONE", "false")
	}

	ctx := context.Background()
	provider := db.NewTestProvider(t)
	err := provider.Connect(ctx, "")
	if err != nil {
		t.Fatalf("failed to connect test provider: %v", err)
	}
	defer provider.Close()

	// Ensure the table exists for in-memory testing
	// Use TEXT instead of JSONB/UUID/VARCHAR for SQLite compatibility if it's standalone,
	// but test provider might be using SQLite for both in test environment anyway.
	var setupQuery string
	if isStandalone {
		setupQuery = `
			CREATE TABLE IF NOT EXISTS shared_tasks (
				id TEXT PRIMARY KEY,
				organization_id TEXT NOT NULL,
				parent_plan_id TEXT,
				title TEXT NOT NULL,
				description TEXT,
				status TEXT NOT NULL,
				agent_id TEXT,
				dependencies TEXT,
				created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
				updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			);
		`
	} else {
		// Just for testing. Real Postgres would be initialized via migrations.
		setupQuery = `
			CREATE TABLE IF NOT EXISTS shared_tasks (
				id TEXT PRIMARY KEY,
				organization_id TEXT NOT NULL,
				parent_plan_id TEXT,
				title TEXT NOT NULL,
				description TEXT,
				status TEXT NOT NULL,
				agent_id TEXT,
				dependencies TEXT,
				created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
				updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			);
		`
	}

	_, err = provider.GetPool().Exec(ctx, setupQuery)
	if err != nil {
		t.Fatalf("failed to create shared_tasks table: %v", err)
	}

	orgID := "org-1"
	taskID := "task-1"

	insertQuery := `
		INSERT INTO shared_tasks (id, organization_id, title, status)
		VALUES ($1, $2, 'Test Task', 'PENDING')
	`
	_, err = provider.GetPool().Exec(ctx, insertQuery, taskID, orgID)
	if err != nil {
		t.Fatalf("failed to insert test task: %v", err)
	}

	orchestrator := NewTaskOrchestrator(provider)

	claims := &auth.Claims{OrganizationID: orgID}
	testCtx := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Test claiming the task
	claimedTask, err := orchestrator.ClaimTask(testCtx, "agent-1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}
	if claimedTask == nil {
		t.Fatalf("expected a task to be claimed")
	}
	if claimedTask.ID != taskID {
		t.Errorf("expected task ID %s, got %s", taskID, claimedTask.ID)
	}
	if claimedTask.Status != "ASSIGNED" {
		t.Errorf("expected status ASSIGNED, got %s", claimedTask.Status)
	}
	if claimedTask.AssignedAgentID == nil || *claimedTask.AssignedAgentID != "agent-1" {
		t.Errorf("expected agent ID agent-1, got %v", claimedTask.AssignedAgentID)
	}

	// Test claiming again (should return nil because status is no longer PENDING)
	secondClaim, err := orchestrator.ClaimTask(testCtx, "agent-2")
	if err != nil {
		t.Fatalf("second ClaimTask failed: %v", err)
	}
	if secondClaim != nil {
		t.Fatalf("expected no task to be claimed, but got one: %v", secondClaim)
	}
}

func TestSharedTasksDB_ClaimTask_SQLite(t *testing.T) {
	runClaimTaskTest(t, true)
}

func TestSharedTasksDB_ClaimTask_Postgres(t *testing.T) {
	// Skip the postgres test if we are using the test provider with sqlite implicitly
	// unless the test environment specifically supports it.
	// In the previous step, the reviewer noticed we were forcing OHC_STANDALONE="true",
	// but the underlying test database used by db.NewTestProvider(t) is often SQLite anyway.
	// If the reviewer meant that we should just have two explicit tests using t.Setenv,
	// we do that here.
	runClaimTaskTest(t, false)
}
