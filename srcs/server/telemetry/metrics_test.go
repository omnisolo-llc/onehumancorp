package telemetry

import (
	"context"
	"testing"
)

func TestRecordTokenBurnRatePredicted24h(ctx *testing.T) {
	err := RecordTokenBurnRatePredicted24h(context.Background(), "test-tenant", "cloud", 1.0)
	if err != nil {
		ctx.Fatalf("expected no error, got %v", err)
	}
}

func TestRecordTokenBudgetAlert(ctx *testing.T) {
	err := RecordTokenBudgetAlert(context.Background(), "test-tenant", "cloud")
	if err != nil {
		ctx.Fatalf("expected no error, got %v", err)
	}
}
