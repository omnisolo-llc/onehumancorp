package telemetry

import (
	"testing"
)

func TestCostTracker(t *testing.T) {
	catalog := map[string]Price{
		"gpt-4": {
			InputPerMillionUSD:  30.0,
			OutputPerMillionUSD: 60.0,
			CachedPerMillionUSD: 15.0,
		},
	}

	tracker := NewCostTracker(catalog)

	tracker.AddUsage("gpt-4", 1000, 500, 200)

	prompt, completion, total, cost := tracker.GetMetrics()

	if prompt != 1000 {
		t.Errorf("expected 1000 prompt tokens, got %d", prompt)
	}
	if completion != 500 {
		t.Errorf("expected 500 completion tokens, got %d", completion)
	}
	if total != 1700 {
		t.Errorf("expected 1700 total tokens, got %d", total)
	}

	expectedCost := (1000.0/1000000.0)*30.0 + (500.0/1000000.0)*60.0 + (200.0/1000000.0)*15.0
	if cost != expectedCost {
		t.Errorf("expected cost %f, got %f", expectedCost, cost)
	}
}
