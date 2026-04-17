package cost_auditor
import (
  "context"
  "fmt"
  "math"
  "sync"
  "ohc/lib/pricing/token_calculator"
)
type AuditEvent struct {
  AgentID              string
  InputTokens          int
  OutputTokens         int
  CachedInputTokens    int
  LocalEmbeddingTokens int
}

type ComputeEvent struct {
  AgentID       string
  DurationHours float64
}

type NetworkEvent struct {
  AgentID   string
  NetworkGB float64
}

type CostAuditor struct {
  mu             sync.Mutex
  config         token_calculator.CostConfig
  agentCosts     map[string]float64
  totalCost      float64
  cachingSavings float64
  storageSavings float64
  computeCost    float64
  networkCost    float64
}
func NewCostAuditor(config token_calculator.CostConfig) *CostAuditor {
  return &CostAuditor{
    config:     config,
    agentCosts: make(map[string]float64),
  }
}
func (a *CostAuditor) RecordComputeEvent(ctx context.Context, event ComputeEvent) float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  cost := token_calculator.CalculateComputeCost(event.DurationHours, a.config)
  a.agentCosts[event.AgentID] += cost
  a.totalCost += cost
  a.computeCost += cost
  return cost
}

func (a *CostAuditor) RecordNetworkEvent(ctx context.Context, event NetworkEvent) float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  cost := token_calculator.CalculateNetworkCost(event.NetworkGB, a.config)
  a.agentCosts[event.AgentID] += cost
  a.totalCost += cost
  a.networkCost += cost
  return cost
}

func (a *CostAuditor) RecordEvent(ctx context.Context, event AuditEvent) float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  cost := token_calculator.CalculateCost(event.InputTokens, event.OutputTokens, event.CachedInputTokens, event.LocalEmbeddingTokens, a.config)
  a.agentCosts[event.AgentID] += cost
  a.totalCost += cost
  return cost
}
func (a *CostAuditor) RecordCacheHit(ctx context.Context, event AuditEvent) float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  actualCost := token_calculator.CalculateCost(event.InputTokens, event.OutputTokens, event.CachedInputTokens, event.LocalEmbeddingTokens, a.config)
  uncachedCost := token_calculator.CalculateCost(event.InputTokens + event.CachedInputTokens, event.OutputTokens, 0, event.LocalEmbeddingTokens, a.config)
  savedCost := math.Round((uncachedCost-actualCost)*10000) / 10000
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

func (a *CostAuditor) RecordStorageCompression(ctx context.Context, originalBytes, compressedBytes int64) float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  savings := token_calculator.CalculateStorageSavings(originalBytes, compressedBytes, a.config)
  a.storageSavings += savings
  return savings
}

func (a *CostAuditor) GetTotalStorageSavings() float64 {
  a.mu.Lock()
  defer a.mu.Unlock()
  return a.storageSavings
}
func (a *CostAuditor) GenerateReport() string {
  a.mu.Lock()
  defer a.mu.Unlock()
  report := fmt.Sprintf("Total Cost: $%.4f\n", a.totalCost)
  report += fmt.Sprintf("Total Compute Cost: $%.4f\n", a.computeCost)
  report += fmt.Sprintf("Total Network Cost: $%.4f\n", a.networkCost)
  report += fmt.Sprintf("Total Savings via Caching: $%.4f\n", a.cachingSavings)
  report += fmt.Sprintf("Total Savings via Storage Compression: $%.4f\n", a.storageSavings)
  report += "Agent Costs:\n"
  for agentID, cost := range a.agentCosts {
    report += fmt.Sprintf("- %s: $%.4f\n", agentID, cost)
  }
  return report
}
