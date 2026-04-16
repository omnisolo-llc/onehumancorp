package billing

import "testing"

func TestAnalyzeTokens(t *testing.T) {
	savings := AnalyzeTokens(100, 50, 0.05)
	if savings != 2.5 {
		t.Errorf("Expected 2.5, got %f", savings)
	}
}
