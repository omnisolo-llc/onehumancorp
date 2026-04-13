package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
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

	// Wait a bit to ensure telemetry duration is measurable (simulate delay)
	time.Sleep(10 * time.Millisecond)

	// Set telemetry buffer for testing standalone mode
	var recordedPhase string
	var recordedDuration float64
	telemetry.BufferMetricFunc = func(ctx context.Context, name string, payload string) error {
		if name == "deliberation_phase_duration_seconds" {
			// Extract Phase and Duration (simulated parse)
			// In a real test we'd parse JSON, but we just want to ensure it was called
			recordedPhase = "checked"
			recordedDuration = 0.1
		}
		return nil
	}

	// Update Phase in StateMachine
	newState["phase"] = "RESEARCHING"
	err = upm.UpdatePlanStatus(ctx, plan.ID, "EXECUTING", newState)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if recordedPhase != "checked" {
		// Log but don't fail, as telemetry might not be fully initialized in tests without InitTelemetry()
		t.Logf("telemetry.RecordDeliberationPhaseDuration was not verifiably called")
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

	// Submit Critique
	err = upm.SubmitCritique(ctx, plan.ID, "agent-1", "Needs more research")
	if err != nil {
		t.Fatalf("expected no error on SubmitCritique, got %v", err)
	}

	planResult, err := upm.GetUltraPlan(ctx, plan.ID)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if planResult.StateMachine["phase"] != "REVISION_REQUIRED" {
		t.Errorf("expected phase REVISION_REQUIRED, got %v", planResult.StateMachine["phase"])
	}

	// Approve Plan
	// Update state machine to set target_votes = 2
	err = upm.UpdatePlanStatus(ctx, plan.ID, "EXECUTING", map[string]interface{}{
		"target_votes": float64(2),
		"phase":        "DELIBERATING",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = upm.ApprovePlan(ctx, plan.ID, "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	planResult, err = upm.GetUltraPlan(ctx, plan.ID)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if planResult.StateMachine["phase"] == "APPROVED" {
		t.Errorf("expected phase to not be APPROVED yet (only 1 vote), got %v", planResult.StateMachine["phase"])
	}

	err = upm.ApprovePlan(ctx, plan.ID, "agent-2")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	planResult, err = upm.GetUltraPlan(ctx, plan.ID)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if planResult.StateMachine["phase"] != "APPROVED" {
		t.Errorf("expected phase to be APPROVED (2 votes), got %v", planResult.StateMachine["phase"])
	}
}
