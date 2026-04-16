package pricing

import (
	"strings"
	"testing"
)

func TestEstimateTokens(t *testing.T) {
	prompt := "hello world"
	tokens := estimateTokens(prompt)
	if tokens == 0 {
		t.Errorf("Expected tokens to be > 0")
	}
}

func TestCostAwareRouter(t *testing.T) {
	budget := NewBudgetManager(1.0)
	router := NewCostAwareRouter(budget, "premium-model", "cheaper-model", 0.01, 0.001)

	// Simple prompt, should route to cheaper model even with budget
	simplePrompt := "hello"
	model, cost, complex := router.Route(simplePrompt)
	if complex {
		t.Errorf("Expected simple prompt")
	}
	if model != "cheaper-model" {
		t.Errorf("Expected cheaper-model, got %s", model)
	}
	if cost == 0 {
		t.Errorf("Expected cost > 0")
	}

	// Complex prompt, should route to premium model with budget
	complexPrompt := strings.Repeat("hello ", 150)
	model, cost, complex = router.Route(complexPrompt)
	if !complex {
		t.Errorf("Expected complex prompt")
	}
	// budget is 1.0, 150 words ~ 200 tokens. Premium cost = 200 * 0.01 = 2.0. This exceeds budget 1.0!
	// Wait, 200 * 0.01 = 2.0. Remaining is 1.0. So it should route to cheaper model!
	if model != "cheaper-model" {
		t.Errorf("Expected cheaper-model, got %s", model)
	}

	// Let's increase budget so it uses premium
	budget = NewBudgetManager(5.0)
	router.Budget = budget
	model, cost, complex = router.Route(complexPrompt)
	if !complex {
		t.Errorf("Expected complex prompt")
	}
	if model != "premium-model" {
		t.Errorf("Expected premium-model, got %s", model)
	}
	if cost == 0 {
		t.Errorf("Expected cost > 0")
	}

	// Record spend to deplete budget
	_, _ = budget.RecordSpend(4.5) // remaining is 0.5
	model, cost, complex = router.Route(complexPrompt)
	if !complex {
		t.Errorf("Expected complex prompt")
	}
	// Premium cost is still ~ 2.0, which > 0.5. So cheaper model.
	if model != "cheaper-model" {
		t.Errorf("Expected cheaper-model, got %s", model)
	}
	if cost == 0 {
		t.Errorf("Expected cost > 0")
	}
}
