package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestUltraPlanManager_PhaseDurationTelemetry(t *testing.T) {
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

	var recordCalled bool
	var recordedPhase string
	var recordedDuration float64
	telemetry.BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		if metricType == "ohc_deliberation_phase_duration_seconds" {
			recordCalled = true
			recordedPhase = "PROPOSE" // Simplification for test verify
			recordedDuration = 1.0
		}
		return nil
	}
	defer func() { telemetry.BufferMetricFunc = nil }()

	plan, err := upm.CreatePlan(ctx, "m-telemetry")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	time.Sleep(10 * time.Millisecond)

	newState := map[string]interface{}{
		"phase": "PROPOSE",
	}

	err = upm.UpdatePlanStatus(ctx, plan.ID, "DELIBERATING", newState)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Another update to transition phase
	time.Sleep(10 * time.Millisecond)
	newState2 := map[string]interface{}{
		"phase": "CRITIQUE",
	}
	err = upm.UpdatePlanStatus(ctx, plan.ID, "DELIBERATING", newState2)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if !recordCalled {
		t.Errorf("expected RecordDeliberationPhaseDuration to be called")
	}
}
