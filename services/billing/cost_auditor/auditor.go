package cost_auditor
import (
  "context"
  "fmt"
  "sync"
  "ohc/lib/pricing/token_calculator"
)
type AuditEvent struct {
  AgentID      string
  InputTokens  int
  OutputTokens int
  CachedTokens int
}
type StorageEvent struct {
  AgentID         string
  OriginalBytes   int
  CompressedBytes int
  CostPerByte     float64
}
type CostAuditor struct {
  mu            sync.Mutex
  config        token_calculator.CostConfig
  agentCosts    map[string]float64
  totalCost     float64
  cachingSavings float64
  storageSavings float64
}
func NewCostAuditor(config token_calculator.CostConfig) *CostAuditor {
  return &CostAuditor{
    config:     config,
    agentCosts: make(map[string]float64),
  }
}
func (a *CostAuditor) RecordEvent(ctx context.Context, event AuditEvent) float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  cost := token_calculator.CalculateCost(event.InputTokens, event.OutputTokens, a.config)
  a.agentCosts[event.AgentID] += cost
  a.totalCost += cost
  return cost
}
func (a *CostAuditor) RecordCacheHit(ctx context.Context, event AuditEvent) float64 {
  a.mu.Lock()
  defer a.mu.Unlock()

  costWithoutCache := token_calculator.CalculateCost(event.InputTokens + event.CachedTokens, event.OutputTokens, a.config)

  costWithCache := token_calculator.CalculateCostWithCache(event.InputTokens, event.CachedTokens, event.OutputTokens, a.config)

  savedCost := costWithoutCache - costWithCache
  if savedCost > 0 {
      a.cachingSavings += savedCost
  }

  a.agentCosts[event.AgentID] += costWithCache
  a.totalCost += costWithCache
  return savedCost
}
func (a *CostAuditor) RecordStorageCompression(ctx context.Context, event StorageEvent) float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  savings := float64(event.OriginalBytes - event.CompressedBytes) * event.CostPerByte
  if savings > 0 {
    a.storageSavings += savings
  }
  return savings
}
func (a *CostAuditor) GetAgentCost(agentID string) float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  return a.agentCosts[agentID]
}
func (a *CostAuditor) GetTotalSavings() float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  return a.cachingSavings
}
func (a *CostAuditor) GetStorageSavings() float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  return a.storageSavings
}
func (a *CostAuditor) GenerateReport() string {
  a.mu.Lock()
  defer a.mu.Unlock()
  report := fmt.Sprintf("Total Cost: $%.4f\n", a.totalCost)
  report += fmt.Sprintf("Total Savings via Caching: $%.4f\n", a.cachingSavings)
  report += fmt.Sprintf("Total Savings via Storage Compression: $%.4f\n", a.storageSavings)
  report += "Agent Costs:\n"
  for agentID, cost := range a.agentCosts {
    report += fmt.Sprintf("- %s: $%.4f\n", agentID, cost)
  }
  return report
}
