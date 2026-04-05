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
	plan, err := upm.CreatePlan(ctx, "m-123", nil)
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
}

func TestUltraPlanManager_SubmitCritique(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	defer prov.Close()

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

	plan, err := upm.CreatePlan(ctx, "m-critique", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = upm.SubmitCritique(ctx, plan.ID, "agent-1", "Needs more caching")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	updatedPlan, err := upm.GetUltraPlan(ctx, plan.ID)
	if err != nil {
		t.Fatalf("expected no error getting plan, got %v", err)
	}

	if updatedPlan.Status != "DELIBERATING" {
		t.Errorf("expected DELIBERATING, got %s", updatedPlan.Status)
	}

	phase, _ := updatedPlan.StateMachine["phase"].(string)
	if phase != "REVISION_REQUIRED" {
		t.Errorf("expected REVISION_REQUIRED, got %s", phase)
	}

	critiques, ok := updatedPlan.StateMachine["critiques"].([]interface{})
	if !ok || len(critiques) != 1 {
		t.Fatalf("expected 1 critique, got %v", updatedPlan.StateMachine["critiques"])
	}

	critiqueMap, ok := critiques[0].(map[string]interface{})
	if !ok || critiqueMap["agent_id"] != "agent-1" || critiqueMap["critique"] != "Needs more caching" {
		t.Errorf("unexpected critique data: %v", critiqueMap)
	}
}

func TestUltraPlanManager_ApprovePlan(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	defer prov.Close()

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

	stateMachine := map[string]interface{}{
		"target_votes": 2.0, // json unmarshals numbers to float64
	}
	plan, err := upm.CreatePlan(ctx, "m-approve", stateMachine)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// First approval, should stay DELIBERATING
	err = upm.ApprovePlan(ctx, plan.ID, "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	updatedPlan, _ := upm.GetUltraPlan(ctx, plan.ID)
	if updatedPlan.Status != "DELIBERATING" {
		t.Errorf("expected DELIBERATING, got %s", updatedPlan.Status)
	}
	if updatedPlan.StateMachine["approvals"].(float64) != 1.0 {
		t.Errorf("expected 1 approval, got %v", updatedPlan.StateMachine["approvals"])
	}

	// Second approval, should reach target and transition to EXECUTING
	err = upm.ApprovePlan(ctx, plan.ID, "agent-2")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	updatedPlan, _ = upm.GetUltraPlan(ctx, plan.ID)
	if updatedPlan.Status != "EXECUTING" {
		t.Errorf("expected EXECUTING, got %s", updatedPlan.Status)
	}
	if updatedPlan.StateMachine["approvals"].(float64) != 2.0 {
		t.Errorf("expected 2 approvals, got %v", updatedPlan.StateMachine["approvals"])
	}
	phase, _ := updatedPlan.StateMachine["phase"].(string)
	if phase != "APPROVED" {
		t.Errorf("expected APPROVED, got %s", phase)
	}
}
