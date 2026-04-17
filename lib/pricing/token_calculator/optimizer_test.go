package token_calculator

import (
	"testing"
)

func TestFindCheapestModel(t *testing.T) {
	models := []ModelPricing{
		{Name: "model-a", CostPerInputToken: 0.01, CostPerOutputToken: 0.02, CostPerCachedInputToken: 0.005},
		{Name: "model-b", CostPerInputToken: 0.005, CostPerOutputToken: 0.03, CostPerCachedInputToken: 0.002},
	}

	cheapest := FindCheapestModel(100, 100, 0, models)
	if cheapest != "model-a" {
		t.Errorf("expected model-a, got %s", cheapest)
	}

	cheapestCached := FindCheapestModel(0, 100, 1000, models)
	if cheapestCached != "model-b" {
		t.Errorf("expected model-b, got %s", cheapestCached)
	}
}
