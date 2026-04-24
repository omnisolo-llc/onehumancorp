package cost_auditor

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/src/server/lib/pricing/tokencalc"
	"math"
	"sync"
)

type AuditEvent struct {
	AgentID              string
	InputTokens          int
	OutputTokens         int
	CachedInputTokens    int
	LocalEmbeddingTokens int
}

type ComputeEvent struct {
	AgentID            string
	ComputeHours       float64
	NetworkEgressBytes int64
}
type CostAuditor struct {
	mu               sync.Mutex
	config           token_calculator.CostConfig
	agentCosts       map[string]float64
	agentBudgets     map[string]float64
	totalCost        float64
	cachingSavings   float64
	storageSavings   float64
	totalComputeCost float64
	totalNetworkCost float64
}

func NewCostAuditor(config token_calculator.CostConfig) *CostAuditor {
	return &CostAuditor{
		config:       config,
		agentCosts:   make(map[string]float64),
		agentBudgets: make(map[string]float64),
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
	uncachedCost := token_calculator.CalculateCost(event.InputTokens+event.CachedInputTokens, event.OutputTokens, 0, event.LocalEmbeddingTokens, a.config)
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

func (a *CostAuditor) RecordComputeEvent(ctx context.Context, event ComputeEvent) float64 {
	a.mu.Lock()
	defer a.mu.Unlock()
	computeCost := token_calculator.CalculateComputeCost(event.ComputeHours, a.config)
	networkCost := token_calculator.CalculateNetworkCost(event.NetworkEgressBytes, a.config)
	totalCost := computeCost + networkCost

	a.agentCosts[event.AgentID] += totalCost
	a.totalCost += totalCost
	a.totalComputeCost += computeCost
	a.totalNetworkCost += networkCost
	return totalCost
}
func (a *CostAuditor) GenerateReport() string {
	a.mu.Lock()
	defer a.mu.Unlock()
	report := fmt.Sprintf("Total Cost: $%.4f\n", a.totalCost)
	report += fmt.Sprintf("Total Savings via Caching: $%.4f\n", a.cachingSavings)
	report += fmt.Sprintf("Total Savings via Storage Compression: $%.4f\n", a.storageSavings)
	report += fmt.Sprintf("Total Compute Cost: $%.4f\n", a.totalComputeCost)
	report += fmt.Sprintf("Total Network Cost: $%.4f\n", a.totalNetworkCost)
	report += "Agent Costs:\n"
	for agentID, cost := range a.agentCosts {
		budget, hasBudget := a.agentBudgets[agentID]
		if hasBudget && cost > budget {
			report += fmt.Sprintf("- %s: $%.4f (OVER BUDGET)\n", agentID, cost)
		} else {
			report += fmt.Sprintf("- %s: $%.4f\n", agentID, cost)
		}
	}
	return report
}

func (a *CostAuditor) SetAgentBudget(agentID string, budget float64) {
	a.mu.Lock()
	defer a.mu.Unlock()
	a.agentBudgets[agentID] = budget
}

func (a *CostAuditor) IsAgentOverBudget(agentID string) bool {
	a.mu.Lock()
	defer a.mu.Unlock()
	cost := a.agentCosts[agentID]
	budget, exists := a.agentBudgets[agentID]
	if !exists {
		return false
	}
	return cost > budget
}
