package cost_auditor
import (
  "context"
  "testing"
  "ohc/lib/pricing/token_calculator"
)
func TestCostAuditor(t *testing.T) {
  config := token_calculator.CostConfig{
    CostPerInputToken:  0.00001,
    CostPerOutputToken: 0.00003,
    DiscountFactor:     0.0,
  }
  auditor := NewCostAuditor(config)
  ctx := context.Background()
  auditor.RecordEvent(ctx, AuditEvent{
    AgentID:      "miser-1",
    InputTokens:  1000,
    OutputTokens: 500,
  })
  cost := auditor.GetAgentCost("miser-1")
  expected := 0.025
  if cost != expected {
    t.Errorf("expected %f, got %f", expected, cost)
  }
  auditor.RecordCacheHit(ctx, AuditEvent{
    AgentID:      "miser-1",
    InputTokens:  1000,
    OutputTokens: 500,
  })
  savings := auditor.GetTotalSavings()
  if savings != expected {
    t.Errorf("expected savings %f, got %f", expected, savings)
  }
  report := auditor.GenerateReport()
  if report == "" {
    t.Errorf("expected non-empty report")
  }
}
