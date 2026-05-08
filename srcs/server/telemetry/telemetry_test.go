package telemetry

import (
	"context"
	"testing"
)

func TestRecordAgentCostEstimateUSD(t *testing.T) {
	ctx := context.Background()
	err := RecordAgentCostEstimateUSD(ctx, "tenant-1", "agent-1", "role-1", 0.5)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestRecordMissionCostCents(t *testing.T) {
	ctx := context.Background()
	err := RecordMissionCostCents(ctx, "tenant-1", "mission-1", "agent-1", "role-1", 50.0)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}
