package growth

import (
	"context"
	"testing"
)

func TestAssignVariant(t *testing.T) {
	variants := []string{"A", "B"}
	weights := []int{100, 0} // Always A

	assigned := AssignVariant(context.Background(), "test_exp", variants, weights)
	if assigned != "A" {
		t.Errorf("Expected A, got %s", assigned)
	}

	weightsB := []int{0, 100} // Always B
	assignedB := AssignVariant(context.Background(), "test_exp", variants, weightsB)
	if assignedB != "B" {
		t.Errorf("Expected B, got %s", assignedB)
	}

	assignedEmpty := AssignVariant(context.Background(), "empty", []string{}, []int{})
	if assignedEmpty != "control" {
		t.Errorf("Expected control fallback, got %s", assignedEmpty)
	}
}

func TestTrackConversion(t *testing.T) {
	// Simple test to ensure it doesn't panic
	TrackConversion(context.Background(), "test_exp", "A")
}
