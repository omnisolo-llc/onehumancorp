package pricing

import (
	"testing"
)

func TestCostOptimizer_AnalyzeCost(t *testing.T) {
	optimizer := NewCostOptimizer(10.0)
	cost := optimizer.AnalyzeCost(1000)

	expected := 10.1
	if cost != expected {
		t.Errorf("Expected cost %.2f, got %.2f", expected, cost)
	}
}

func TestCostOptimizer_GetTokenEfficiency(t *testing.T) {
	optimizer := NewCostOptimizer(10.0)
	msg := optimizer.GetTokenEfficiency()

	expected := "Efficiency calculated for base cost: 10.00"
	if msg != expected {
		t.Errorf("Expected message %q, got %q", expected, msg)
	}
}

func TestCostOptimizer_Caching(t *testing.T) {
	optimizer := NewCostOptimizer(10.0)
	prompt := "Translate hello to french"
	response := "bonjour"

	_, found := optimizer.GetCachedPrompt(prompt)
	if found {
		t.Errorf("Expected cache miss for new prompt")
	}

	optimizer.SetCachedPrompt(prompt, response)

	cachedResponse, found := optimizer.GetCachedPrompt(prompt)
	if !found {
		t.Errorf("Expected cache hit for cached prompt")
	}
	if cachedResponse != response {
		t.Errorf("Expected cached response %q, got %q", response, cachedResponse)
	}
}

func TestEstimateTokens(t *testing.T) {
	optimizer := NewCostOptimizer(0.0)
	text := "This is a test string." // 22 chars

	tokens := optimizer.EstimateTokens(text)
	if tokens != 5 {
		t.Fatalf("Expected 5 tokens, got %d", tokens)
	}
}
