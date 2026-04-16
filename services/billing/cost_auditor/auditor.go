package cost_auditor
import (
  "context"
  "fmt"
  "math"
  "sync"
  "github.com/onehumancorp/mono/lib/pricing/token_calculator"
)
type StorageEvent struct {
	AgentID         string
	OriginalBytes   int64
	CompressedBytes int64
}

type AuditEvent struct {
  AgentID              string
  InputTokens          int
  OutputTokens         int
  CachedInputTokens    int
  LocalEmbeddingTokens int
}
type CostAuditor struct {
  mu             sync.Mutex
  config         token_calculator.CostConfig
  agentCosts     map[string]float64
  totalCost      float64
  cachingSavings float64
	storageSavings float64
	totalSavedBytes int64
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

func (a *CostAuditor) RecordStorageCompression(ctx context.Context, event StorageEvent) float64 {
	a.mu.Lock()
	defer a.mu.Unlock()

	savings := token_calculator.CalculateStorageSavings(event.OriginalBytes, event.CompressedBytes, a.config)
	a.storageSavings += savings

	if event.OriginalBytes > event.CompressedBytes {
		a.totalSavedBytes += (event.OriginalBytes - event.CompressedBytes)
	}

	return savings
}

func (a *CostAuditor) GetTotalStorageSavings() float64 {
	a.mu.Lock()
	defer a.mu.Unlock()
	return a.storageSavings
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
	report += fmt.Sprintf("Total Savings via Storage Compression: $%.4f (Saved: %d Bytes)\n", a.storageSavings, a.totalSavedBytes)
  report += "Agent Costs:\n"
  for agentID, cost := range a.agentCosts {
    report += fmt.Sprintf("- %s: $%.4f\n", agentID, cost)
  }
  return report
}
