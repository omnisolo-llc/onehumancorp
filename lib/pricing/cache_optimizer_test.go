package pricing

import "testing"

func TestCalculateSavings(t *testing.T) {
	optimizer := NewCacheOptimizer(0.01)
	savings := optimizer.CalculateSavings(100, 20)
	if savings != 0.8 {
		t.Errorf("Expected 0.8, got %f", savings)
	}
}
