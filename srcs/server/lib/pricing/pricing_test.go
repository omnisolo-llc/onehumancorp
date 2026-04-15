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

func TestCostOptimizer_OptimizePrompt(t *testing.T) {
	optimizer := NewCostOptimizer(10.0)

	simplePrompt := "This is a long prompt and it has a lot of words to test the truncation limit."

	optimized := optimizer.OptimizePrompt(simplePrompt, 5)

	// ReduceTokens removes "is", "a", "and", "a", "of", "to", "the"
	// Original words: This is a long prompt and it has a lot of words to test the truncation limit.
	// After ReduceTokens: This long prompt it has lot words test truncation limit.
	// After TruncateByWordCount(5): This long prompt it has

	expected := "This long prompt it has"
	if optimized != expected {
		t.Errorf("Expected optimized prompt %q, got %q", expected, optimized)
	}

	jsonPrompt := `{ "foo": "bar" }`
	optimizedJson := optimizer.OptimizePrompt(jsonPrompt, 5)
	// MinifyJSONPrompt output: {"foo":"bar"}
	// Then ReduceTokens: {"foo":"bar"}
	// Then TruncateByWordCount(5): {"foo":"bar"}
	expectedJson := `{"foo":"bar"}`
	if optimizedJson != expectedJson {
		t.Errorf("Expected optimized json %q, got %q", expectedJson, optimizedJson)
	}
}
