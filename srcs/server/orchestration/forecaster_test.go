package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestTokenBurnForecaster(t *testing.T) {
	hub := NewHub()
	defer hub.Close()

	agent := Agent{
		ID:             "agent1",
		Role:           "worker",
		OrganizationID: "org-xyz",
		Status:         StatusIdle,
	}
	hub.RegisterAgent(agent)

	// Test the callback logic
	// The forecaster is already created inside NewHub
	telemetry.RecordTokenUsage(context.Background(), "agent1", "worker", "gpt-4", "prompt", 1000)

	forecaster := hub.forecaster
	if forecaster == nil {
		t.Fatalf("forecaster is nil")
	}

	// Wait briefly for telemetry callback to execute
	time.Sleep(100 * time.Millisecond)

	forecaster.mu.Lock()
	count := forecaster.usage["org-xyz"]
	forecaster.mu.Unlock()

	if count != int64(1000) {
		t.Errorf("expected 1000, got %d", count)
	}

	// Test runForecastLoop functionality safely
	// We can't easily jump time for a standard time.Ticker,
	// so we'll directly test the internal loop logic via the exposed method.

	forecaster.mu.Lock()
	forecaster.windowStart = time.Now().Add(-2 * time.Minute)
	forecaster.mu.Unlock()

	forecaster.ProcessForecastTick()

	// Ensure the count was reset correctly
	forecaster.mu.Lock()
	countAfterReset := forecaster.usage["org-xyz"]
	forecaster.mu.Unlock()

	if countAfterReset != 0 {
		t.Errorf("expected usage to reset to 0, got %d", countAfterReset)
	}
}
