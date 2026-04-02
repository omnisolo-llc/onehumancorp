package telemetry_test

import (
	"testing"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestCalculateBurnRate(t *testing.T) {
	tests := []struct {
		name     string
		history  []int64
		expected float64
	}{
		{
			name:     "empty history",
			history:  []int64{},
			expected: 0,
		},
		{
			name:     "single item",
			history:  []int64{100},
			expected: 0,
		},
		{
			name:     "two items",
			history:  []int64{100, 200},
			expected: 100,
		},
		{
			name:     "five items linear",
			history:  []int64{100, 200, 300, 400, 500},
			expected: 100, // (500 - 100) / 4 = 100
		},
		{
			name:     "five items varying",
			history:  []int64{100, 150, 300, 350, 500},
			expected: 100, // (500 - 100) / 4 = 100
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := telemetry.CalculateBurnRate(tt.history)
			if result != tt.expected {
				t.Errorf("CalculateBurnRate() = %v, want %v", result, tt.expected)
			}
		})
	}
}
