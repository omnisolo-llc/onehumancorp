package orchestration

import (
	"context"
	"fmt"
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

type mockMinimaxClient struct {
	reasonResponses []string
	reasonCalls     int
}

func (m *mockMinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	if m.reasonCalls >= len(m.reasonResponses) {
		return "", fmt.Errorf("no more mock responses")
	}
	resp := m.reasonResponses[m.reasonCalls]
	m.reasonCalls++
	return resp, nil
}

func (m *mockMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestUltraPlanDeliberator(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")

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

	mockLLM := &mockMinimaxClient{
		reasonResponses: []string{
			"Proposal 1",
			"Critique 1",
			"Refined Plan 1",
		},
	}

	deliberator := NewUltraPlanDeliberator(upm, mockLLM)

	missionID := "mission-456"
	prompt := "Build an awesome feature"

	plan, err := deliberator.Deliberate(ctx, missionID, prompt)
	if err != nil {
		t.Fatalf("expected no error from Deliberate, got %v", err)
	}

	if plan == nil {
		t.Fatal("expected plan, got nil")
	}

	if plan.MissionID != missionID {
		t.Errorf("expected mission ID %s, got %s", missionID, plan.MissionID)
	}

	if plan.Status != "EXECUTING" {
		t.Errorf("expected status EXECUTING, got %s", plan.Status)
	}

	if plan.StateMachine["phase"] != "APPROVED" {
		t.Errorf("expected phase APPROVED, got %v", plan.StateMachine["phase"])
	}

	if plan.StateMachine["proposal"] != "Proposal 1" {
		t.Errorf("expected proposal 'Proposal 1', got %v", plan.StateMachine["proposal"])
	}

	if plan.StateMachine["final_plan"] != "Refined Plan 1" {
		t.Errorf("expected final_plan 'Refined Plan 1', got %v", plan.StateMachine["final_plan"])
	}

	// Verify file was written
	filepath := fmt.Sprintf(".agent-task/ultraplans/%s.md", missionID)
	content, err := os.ReadFile(filepath)
	if err != nil {
		t.Fatalf("expected file to exist at %s, got err %v", filepath, err)
	}
	if string(content) != "Refined Plan 1" {
		t.Errorf("expected file content 'Refined Plan 1', got %s", string(content))
	}

	// Clean up
	_ = os.Remove(filepath)
}
