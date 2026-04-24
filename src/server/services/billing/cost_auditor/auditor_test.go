package cost_auditor

import (
	"context"
	"github.com/onehumancorp/mono/src/server/lib/pricing/tokencalc"
	"testing"
)

func TestCostAuditor(t *testing.T) {
	config := token_calculator.CostConfig{
		CostPerInputToken:       0.00001,
		CostPerOutputToken:      0.00003,
		CostPerCachedInputToken: 0.000005,
		CostPerLocalEmbedding:   0.000002,
		DiscountFactor:          0.0,
		CostPerGBMonth:          0.023,
		CostPerComputeHour:      0.10,
		CostPerNetworkGB:        0.05,
	}
	auditor := NewCostAuditor(config)
	ctx := context.Background()
	auditor.RecordEvent(ctx, AuditEvent{
		AgentID:              "miser-1",
		InputTokens:          1000,
		OutputTokens:         500,
		CachedInputTokens:    0,
		LocalEmbeddingTokens: 1000,
	})
	cost := auditor.GetAgentCost("miser-1")
	expected := 0.027
	if cost != expected {
		t.Errorf("expected %f, got %f", expected, cost)
	}
	auditor.RecordCacheHit(ctx, AuditEvent{
		AgentID:              "miser-1",
		InputTokens:          1000,
		OutputTokens:         500,
		CachedInputTokens:    2000,
		LocalEmbeddingTokens: 0,
	})
	savings := auditor.GetTotalSavings()
	if savings != 0.01 {
		t.Errorf("expected savings %f, got %f", 0.01, savings)
	}
	savingsStorage := auditor.RecordStorageCompression(ctx, 10737418240, 5368709120)
	if savingsStorage != 0.115 {
		t.Errorf("expected storage savings 0.115, got %f", savingsStorage)
	}
	totalStorageSavings := auditor.GetTotalStorageSavings()
	if totalStorageSavings != 0.115 {
		t.Errorf("expected total storage savings 0.115, got %f", totalStorageSavings)
	}
	auditor.RecordComputeEvent(ctx, ComputeEvent{
		AgentID:            "miser-1",
		ComputeHours:       2.5,
		NetworkEgressBytes: 2147483648, // 2GB
	})

	// Compute cost: 2.5 * 0.10 = 0.25
	// Network cost: 2 * 0.05 = 0.10
	// Total additional cost: 0.35
	// Previous total cost for miser-1: 0.027
	// New total cost for miser-1: 0.377

	newAgentCost := auditor.GetAgentCost("miser-1")
	expectedNewAgentCost := 0.377
	if newAgentCost != expectedNewAgentCost {
		t.Errorf("expected new agent cost %f, got %f", expectedNewAgentCost, newAgentCost)
	}

	report := auditor.GenerateReport()
	if report == "" {
		t.Errorf("expected non-empty report")
	}
}

func TestBudget(t *testing.T) {
	config := token_calculator.CostConfig{
		CostPerInputToken:  0.00001,
		CostPerOutputToken: 0.00003,
	}
	auditor := NewCostAuditor(config)
	ctx := context.Background()

	auditor.SetAgentBudget("miser-2", 0.01)

	auditor.RecordEvent(ctx, AuditEvent{
		AgentID:      "miser-2",
		InputTokens:  1000,
		OutputTokens: 500,
	})

	if !auditor.IsAgentOverBudget("miser-2") {
		t.Errorf("expected miser-2 to be over budget")
	}

	report := auditor.GenerateReport()
	if report == "" {
		t.Errorf("expected non-empty report")
	}
}
