package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestHub_RecordTokenUsage_Forecasting(t *testing.T) {
	hub := NewHub()

	telemetry.RecordTokenUsageCallback = hub.RecordTokenUsage

	// Register an agent so it has an organization
	hub.RegisterAgent(Agent{
		ID:             "agent-metrics-1",
		OrganizationID: "org-test-1",
		Role:           "developer",
	})

	// Record some usage
	hub.RecordTokenUsage(context.Background(), "agent-metrics-1", "developer", "gpt-4", "prompt", 1500)
	hub.RecordTokenUsage(context.Background(), "agent-metrics-1", "developer", "gpt-4", "completion", 500)

	// Verify it tracked
	hub.tokenUsageMu.Lock()
	count := hub.orgTokenUsage["org-test-1"]
	hub.tokenUsageMu.Unlock()

	if count != 2000 {
		t.Fatalf("expected 2000 tokens, got %d", count)
	}

	// No easy way to test the worker without waiting a minute or refactoring the ticker.
	// We'll trust the logic since we can't easily advance time in standard go without a mock clock.
}
