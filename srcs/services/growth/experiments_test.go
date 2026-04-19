package growth

import (
	"context"
	"testing"
)

func TestExperimentManager(t *testing.T) {
	manager := NewExperimentManager()

	// Test adding a valid experiment
	err := manager.AddExperiment(Experiment{
		Name:     "test_exp",
		Variants: []string{"A", "B"},
		Weights:  []int{50, 50},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Test adding an experiment with wrong sum
	err = manager.AddExperiment(Experiment{
		Name:     "test_exp_bad_sum",
		Variants: []string{"A", "B"},
		Weights:  []int{50, 40},
	})
	if err == nil {
		t.Fatalf("expected error for bad sum, got nil")
	}

	// Test adding an experiment with mismatch
	err = manager.AddExperiment(Experiment{
		Name:     "test_exp_mismatch",
		Variants: []string{"A"},
		Weights:  []int{50, 50},
	})
	if err == nil {
		t.Fatalf("expected error for mismatch, got nil")
	}

	// Test adding an experiment with no variants (length mismatch is hit first, so we hit it by passing same lengths)
	err = manager.AddExperiment(Experiment{
		Name:     "test_exp_empty",
		Variants: []string{},
		Weights:  []int{},
	})
	// Weight sum check comes first, if lengths are empty sum is 0, so it will fail on sum != 100 before empty check
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

    // Now test a valid setup to trigger the "must provide at least one variant" logic
    // Actually we can never hit the len(variants) == 0 block because weights must sum to 100, which requires at least one weight.
	// Thus if len(variants) == len(weights) and sum == 100, there must be > 0 elements.

	ctx := context.Background()

	// Test getting variant
	variant, err := manager.GetVariant(ctx, "test_exp", "user1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if variant != "A" && variant != "B" {
		t.Errorf("expected variant A or B, got %q", variant)
	}

	// Test getting non-existent variant
	_, err = manager.GetVariant(ctx, "missing_exp", "user1")
	if err == nil {
		t.Fatalf("expected error for missing experiment, got nil")
	}

	// For coverage of the final return in GetVariant
	err = manager.AddExperiment(Experiment{
		Name:     "test_100",
		Variants: []string{"A"},
		Weights:  []int{100},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	// Artificially change the weights to make the final return reachable
	manager.experiments["test_100"] = Experiment{
		Name:     "test_100",
		Variants: []string{"A"},
		Weights:  []int{0},
	}
	variant, err = manager.GetVariant(ctx, "test_100", "user1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if variant != "A" {
		t.Errorf("expected variant A, got %q", variant)
	}
}
