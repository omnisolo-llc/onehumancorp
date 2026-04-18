package token_calculator

import (
	"testing"
)

func TestCalculateCost(t *testing.T) {
	config := CostConfig{
		CostPerInputToken:       0.00001,
		CostPerOutputToken:      0.00003,
		CostPerCachedInputToken: 0.000005,
		CostPerLocalEmbedding:   0.000002,
		DiscountFactor:          0.10,
		CostPerGBMonth:          0.023,
	}
	cost := CalculateCost(1000, 500, 2000, 1000, config)
	expected := 0.0333
	if cost != expected {
		t.Errorf("expected %f, got %f", expected, cost)
	}
}

func TestCalculateStorageSavings(t *testing.T) {
	config := CostConfig{
		CostPerGBMonth: 0.023,
	}

	savings := CalculateStorageSavings(10737418240, 5368709120, config) // 10GB -> 5GB
	expected := 0.1150
	if savings != expected {
		t.Errorf("expected %f, got %f", expected, savings)
	}
}

func TestCalculateComputeCost(t *testing.T) {
	config := CostConfig{
		CostPerComputeHour: 0.10,
	}
	cost := CalculateComputeCost(2.5, config)
	expected := 0.2500
	if cost != expected {
		t.Errorf("expected %f, got %f", expected, cost)
	}
}

func TestCalculateNetworkCost(t *testing.T) {
	config := CostConfig{
		CostPerNetworkGB: 0.05,
	}
	cost := CalculateNetworkCost(2147483648, config) // 2GB
	expected := 0.1000
	if cost != expected {
		t.Errorf("expected %f, got %f", expected, cost)
	}
}

func TestCalculateContextCompressionSavings(t *testing.T) {
	config := CostConfig{
		CostPerInputToken: 0.00001,
	}
	savings := CalculateContextCompressionSavings(10000, 2000, config) // 8000 tokens saved
	expected := 0.0800
	if savings != expected {
		t.Errorf("expected %f, got %f", expected, savings)
	}
}
