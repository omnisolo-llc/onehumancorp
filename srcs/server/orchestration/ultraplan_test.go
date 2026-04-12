package orchestration

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestUltraPlanManager(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")

	telemetry.InitTelemetry()
	var loggedMetric string
	var loggedPayload string
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		loggedMetric = metricType
		loggedPayload = payload
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = nil }()

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

	if loggedMetric != "deliberation_phase_duration" {
		t.Errorf("expected deliberation_phase_duration metric to be logged, got %q", loggedMetric)
	}
	if loggedPayload == "" {
		t.Errorf("expected payload to be logged")
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

func TestSyncDAGDependencies(t *testing.T) {
	dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}

	ctx := context.Background()

	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS shared_tasks_v2 (
			id TEXT PRIMARY KEY,
			parent_plan_id TEXT,
			status TEXT,
			dependencies JSONB
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
		INSERT INTO shared_tasks_v2 (id, parent_plan_id, status, dependencies) VALUES
		('task1', 'plan1', 'COMPLETED', '[]'),
		('task2', 'plan1', 'BLOCKED', '["task1"]'),
		('task3', 'plan1', 'BLOCKED', '["task1", "task4"]'),
		('task4', 'plan1', 'PENDING', '[]')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	m := NewUltraPlanManager(dbProvider, nil, nil)
	err = m.SyncDAGDependencies(ctx, "plan1")
	if err != nil {
		t.Fatalf("failed to sync dag dependencies: %v", err)
	}

	var status2, status3 string
	err = dbProvider.QueryRow(ctx, "SELECT status FROM shared_tasks_v2 WHERE id = 'task2'").Scan(&status2)
	if err != nil {
		t.Fatalf("failed to query task2: %v", err)
	}
	if status2 != "PENDING" {
		t.Fatalf("expected task2 status PENDING, got %s", status2)
	}

	err = dbProvider.QueryRow(ctx, "SELECT status FROM shared_tasks_v2 WHERE id = 'task3'").Scan(&status3)
	if err != nil {
		t.Fatalf("failed to query task3: %v", err)
	}
	if status3 != "BLOCKED" {
		t.Fatalf("expected task3 status BLOCKED, got %s", status3)
	}
}
