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
}
type CostAuditor struct {
  mu            sync.Mutex
  config        token_calculator.CostConfig
  agentCosts    map[string]float64
  totalCost     float64
  cachingSavings float64
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
  savedCost := token_calculator.CalculateCost(event.InputTokens, event.OutputTokens, a.config)
  a.cachingSavings += savedCost
  return savedCost
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
func (a *CostAuditor) GenerateReport() string {
  a.mu.Lock()
  defer a.mu.Unlock()
  report := fmt.Sprintf("Total Cost: $%.4f\n", a.totalCost)
  report += fmt.Sprintf("Total Savings via Caching: $%.4f\n", a.cachingSavings)
  report += "Agent Costs:\n"
  for agentID, cost := range a.agentCosts {
    report += fmt.Sprintf("- %s: $%.4f\n", agentID, cost)
  }
  return report
}
