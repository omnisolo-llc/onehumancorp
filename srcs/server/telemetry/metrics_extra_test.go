package telemetry

import (
	"context"
	"testing"
)

func TestTokenBurnRateForecaster(t *testing.T) {
	// Reset usage map for test
	usageMu.Lock()
	tokenUsage = make(map[string][]usageEvent)
	usageMu.Unlock()

	// Simulate some token usages
	RecordTokenUsage(context.Background(), "agent-1", "role", "model", "prompt", 600)
	RecordTokenUsage(context.Background(), "agent-1", "role", "model", "prompt", 1200)

	// Since we can't easily assert on telemetry.RecordTokenBurnRate without intercepting,
	// let's verify internal state. (Since this is in the same package, it can access them)
	usageMu.Lock()
	events := tokenUsage["unknown"]
	usageMu.Unlock()

	if len(events) != 2 {
		t.Fatalf("expected 2 events, got %d", len(events))
	}
}
