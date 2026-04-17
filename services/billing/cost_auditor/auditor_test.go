package cost_auditor
import (
  "context"
  "testing"
  "ohc/lib/pricing/token_calculator"
)
func TestCostAuditor(t *testing.T) {
  config := token_calculator.CostConfig{
    CostPerInputToken:       0.00001,
    CostPerOutputToken:      0.00003,
    CostPerCachedInputToken: 0.000005,
    CostPerLocalEmbedding:   0.000002,
    DiscountFactor:          0.0,
    CostPerGBMonth:          0.023,
    CostPerComputeHour:      0.05,
    CostPerNetworkGB:        0.09,
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

  computeCost := auditor.RecordComputeEvent(ctx, ComputeEvent{
    AgentID:       "miser-1",
    DurationHours: 2.0,
  })
  if computeCost != 0.1000 {
    t.Errorf("expected compute cost 0.1000, got %f", computeCost)
  }

  networkCost := auditor.RecordNetworkEvent(ctx, NetworkEvent{
    AgentID:   "miser-1",
    NetworkGB: 1.0,
  })
  if networkCost != 0.0900 {
    t.Errorf("expected network cost 0.0900, got %f", networkCost)
  }

  report := auditor.GenerateReport()
  if report == "" {
    t.Errorf("expected non-empty report")
  }
}
