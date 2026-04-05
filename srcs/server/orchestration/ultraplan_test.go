package orchestration

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestUltraPlanManager(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	// Ensure the table exists in memory DB for the test (test provider only creates some)
	_, _ = prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS swarm_ultra_plans (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'DELIBERATING',
			state_machine TEXT DEFAULT '{}',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)

	upm := NewUltraPlanManager(prov, nil, nil)
	ctx := context.Background()

	// Create
	plan, err := upm.CreatePlan(ctx, "m-123")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if plan == nil {
		t.Fatal("expected plan, got nil")
	}
	if plan.MissionID != "m-123" {
		t.Errorf("expected m-123, got %s", plan.MissionID)
	}
	if plan.Status != "DELIBERATING" {
		t.Errorf("expected DELIBERATING, got %s", plan.Status)
	}

	// Update
	newState := map[string]interface{}{
		"step": "research",
	}
	err = upm.UpdatePlanStatus(ctx, plan.ID, "EXECUTING", newState)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify update
	var status, smStr string
	err = prov.QueryRow(ctx, "SELECT status, state_machine FROM swarm_ultra_plans WHERE id = $1", plan.ID).Scan(&status, &smStr)
	if err != nil {
		t.Fatalf("expected no error reading db, got %v", err)
	}
	if status != "EXECUTING" {
		t.Errorf("expected EXECUTING, got %s", status)
	}

	// Create a second plan for Deliberation testing
	plan2, err := upm.CreatePlan(ctx, "m-456", map[string]interface{}{"target_votes": 2.0})
	if err != nil {
		t.Fatalf("expected no error creating plan2, got %v", err)
	}

	// Submit Critique
	err = upm.SubmitCritique(ctx, plan2.ID, "agent-1", "Needs more security")
	if err != nil {
		t.Fatalf("expected no error from SubmitCritique, got %v", err)
	}

	plan2Updated, err := upm.GetUltraPlan(ctx, plan2.ID)
	if err != nil {
		t.Fatalf("GetUltraPlan failed: %v", err)
	}
	if phase, _ := plan2Updated.StateMachine["phase"].(string); phase != "REVISION_REQUIRED" {
		t.Errorf("expected REVISION_REQUIRED phase, got %v", phase)
	}

	// Approve Plan 1st time
	err = upm.ApprovePlan(ctx, plan2.ID, "agent-2")
	if err != nil {
		t.Fatalf("expected no error from ApprovePlan, got %v", err)
	}

	plan2Updated, err = upm.GetUltraPlan(ctx, plan2.ID)
	if err != nil {
		t.Fatalf("GetUltraPlan failed: %v", err)
	}
	if phase, _ := plan2Updated.StateMachine["phase"].(string); phase == "APPROVED" {
		t.Errorf("expected phase NOT to be APPROVED yet since target is 2")
	}

	// Approve Plan 2nd time
	err = upm.ApprovePlan(ctx, plan2.ID, "agent-3")
	if err != nil {
		t.Fatalf("expected no error from ApprovePlan, got %v", err)
	}

	plan2Updated, err = upm.GetUltraPlan(ctx, plan2.ID)
	if err != nil {
		t.Fatalf("GetUltraPlan failed: %v", err)
	}
	if phase, _ := plan2Updated.StateMachine["phase"].(string); phase != "APPROVED" {
		t.Errorf("expected APPROVED phase, got %v", phase)
	}
}
