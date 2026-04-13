package analytics

import (
	"testing"
)

func TestComputeViralCoefficient(t *testing.T) {
	tests := []struct {
		conversions int
		inviters    int
		expected    float64
	}{
		{6, 3, 2.0},
		{10, 5, 2.0},
		{0, 5, 0.0},
		{10, 0, 0.0},
	}

	for _, tt := range tests {
		got := ComputeViralCoefficient(tt.conversions, tt.inviters)
		if got != tt.expected {
			t.Errorf("ComputeViralCoefficient(%d, %d) = %f; want %f", tt.conversions, tt.inviters, got, tt.expected)
		}
	}
}

func TestCalculateConversionRate(t *testing.T) {
	tests := []struct {
		referrals   int
		conversions int
		expected    float64
	}{
		{100, 10, 10.0},
		{50, 25, 50.0},
		{0, 10, 0.0},
		{100, 0, 0.0},
	}

	for _, tt := range tests {
		got := CalculateConversionRate(tt.referrals, tt.conversions)
		if got != tt.expected {
			t.Errorf("CalculateConversionRate(%d, %d) = %f; want %f", tt.referrals, tt.conversions, got, tt.expected)
		}
	}
}
