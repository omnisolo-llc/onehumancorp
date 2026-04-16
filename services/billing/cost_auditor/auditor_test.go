package cost_auditor
import (
  "context"
  "testing"
  "math"
  "ohc/lib/pricing/token_calculator"
)
func TestCostAuditor(t *testing.T) {
  config := token_calculator.CostConfig{
    CostPerInputToken:       0.00001,
    CostPerCachedInputToken: 0.000005,
    CostPerOutputToken:      0.00003,
    DiscountFactor:          0.0,
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

  savings := auditor.RecordCacheHit(ctx, AuditEvent{
    AgentID:      "miser-1",
    InputTokens:  1000,
    OutputTokens: 500,
    CachedTokens: 1000,
  })

  expectedSavings := 0.005
  if math.Abs(savings - expectedSavings) > 1e-6 {
      t.Errorf("expected savings %f, got %f", expectedSavings, savings)
  }


  auditor.RecordStorageCompression(ctx, StorageEvent{
    AgentID:         "miser-1",
    OriginalBytes:   2000,
    CompressedBytes: 1000,
    CostPerByte:     0.0001,
  })
  storageSavings := auditor.GetStorageSavings()
  expectedStorageSavings := 0.1
  if storageSavings != expectedStorageSavings {
    t.Errorf("expected storage savings %f, got %f", expectedStorageSavings, storageSavings)
  }

  report := auditor.GenerateReport()
  if report == "" {
    t.Errorf("expected non-empty report")
  }
}
