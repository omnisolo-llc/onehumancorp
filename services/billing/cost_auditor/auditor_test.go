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
  report := auditor.GenerateReport()
  if report == "" {
    t.Errorf("expected non-empty report")
  }
}
