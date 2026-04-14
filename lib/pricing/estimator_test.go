package pricing

import (
	"testing"
)

func TestTokenEstimator_EstimateTokens(t *testing.T) {
	estimator := NewTokenEstimator()

	tests := []struct {
		text     string
		expected int
	}{
		{"", 0},
		{"hello", 2},
		{"hello world", 3},
		{"this is a longer sentence to test token estimation", 13},
	}

	for _, tt := range tests {
		got := estimator.EstimateTokens(tt.text)
		if got != tt.expected {
			t.Errorf("EstimateTokens(%q) = %d, expected %d", tt.text, got, tt.expected)
		}
	}
}
