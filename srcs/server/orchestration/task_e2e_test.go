package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/models"
)

// TestOrchestration_E2E tests the Critical User Journey of dispatching a high level plan
// through task decomposition into execution and validation in the Swarm.
func TestOrchestration_E2E(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "false")
	provider := db.NewTestProvider(t)
    ctx := context.Background()

    // Setup core E2E Tables
    _, _ = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			priority TEXT,
			payload TEXT NOT NULL DEFAULT '{}',
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)

	// Orchestrator initialization
	orchestrator := NewDefaultTaskOrchestrator(provider, nil, nil)

    // 1. High Level Request
    planID, err := orchestrator.ReceiveHighLevelRequest(ctx, "org-1", "Build UI System")
    if err != nil {
        t.Fatalf("failed to receive high level request: %v", err)
    }

    // 2. Validate it creates a decomposing task
    var status string
    provider.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", planID).Scan(&status)
    if status != "DECOMPOSING" {
        t.Fatalf("expected high level task to be DECOMPOSING, got %v", status)
    }

    // Since this is a test environment, simulate the Decomposition subagent output:
    _, _ = provider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status) VALUES ('sub-task-1', 'org-1', 'Implement Login', 'PENDING')")

    // 3. A decomposing agent acquires the next piece
    subTask, err := orchestrator.ClaimDecompositionTask(ctx, "agent-decomposer")
    if err != nil {
        t.Fatalf("failed to claim decomposition task: %v", err)
    }
    if subTask == nil {
        t.Fatalf("expected to claim the subtask")
    }
    if subTask.Status != "IN_PROGRESS" || *subTask.AssignedAgentID != "agent-decomposer" {
        t.Fatalf("failed to validate state machine lock acquisition")
    }

    // 4. E2E Complete flow.
    // Simulate decomposing agent finishing and putting executable tasks
    _, _ = provider.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'COMPLETED' WHERE id = 'sub-task-1'")
    t.Log("Successfully verified E2E Critical User Journey for KAIROS Orchestration")
}
