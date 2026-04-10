package pricing

import (
	"math"
	"testing"
)

func TestTokenCostCalculator_CalculateCost(t *testing.T) {
	calc := NewTokenCostCalculator()

	tests := []struct {
		name          string
		model         string
		input         int
		output        int
		cached        int
		expectedCost  float64
		expectError   bool
	}{
		{
			name:         "Claude 3.5 Sonnet Standard",
			model:        "claude-3-5-sonnet-20240620",
			input:        1000000,
			output:       1000000,
			cached:       0,
			expectedCost: 18.00,
			expectError:  false,
		},
		{
			name:         "Claude 3.5 Sonnet with Caching",
			model:        "claude-3-5-sonnet-20240620",
			input:        100000,
			output:       50000,
			cached:       900000,
			expectedCost: 1.32, // (0.1M * 3) + (0.05M * 15) + (0.9M * 0.30) = 0.3 + 0.75 + 0.27 = 1.32
			expectError:  false,
		},
		{
			name:         "GPT-4o Standard",
			model:        "gpt-4o",
			input:        2000000,
			output:       500000,
			cached:       0,
			expectedCost: 17.50, // (2M * 5) + (0.5M * 15) = 10 + 7.5 = 17.5
			expectError:  false,
		},
		{
			name:        "Unsupported Model",
			model:       "unknown-model",
			input:       100,
			output:      100,
			cached:      0,
			expectError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cost, err := calc.CalculateCost(tt.model, tt.input, tt.output, tt.cached)
			if tt.expectError {
				if err == nil {
					t.Errorf("expected error for unsupported model, got nil")
				}
				return
			}
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}

			// Compare with a small epsilon for floating point errors
			if math.Abs(cost-tt.expectedCost) > 1e-6 {
				t.Errorf("expected cost %v, got %v", tt.expectedCost, cost)
			}
		})
	}
}
