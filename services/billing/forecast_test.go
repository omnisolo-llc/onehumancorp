package billing

import (
	"context"
	"math"
	"testing"
	"time"
)

func TestForecaster_TrackEvent_And_ProjectMonthlyCost(t *testing.T) {
	// Use a 1-hour window for testing
	f := NewForecaster(1 * time.Hour)
	tenantID := "test-org"

	// Mocking events manually to bypass time.Now() inside TrackEvent for exact testing
	now := time.Now()

	// Track 10 USD over the last hour
	f.events[tenantID] = []Event{
		{Timestamp: now.Add(-30 * time.Minute), Cost: 5.0},
		{Timestamp: now.Add(-10 * time.Minute), Cost: 5.0},
	}

	// 10 USD per hour * 24 hours * 30 days = 7200 USD
	expectedCost := 10.0 * 24.0 * 30.0

	projection := f.ProjectMonthlyCost(context.Background(), tenantID)

	if math.Abs(projection-expectedCost) > 1e-9 {
		t.Errorf("Expected projection %f, got %f", expectedCost, projection)
	}
}

func TestForecaster_NoEvents(t *testing.T) {
	f := NewForecaster(1 * time.Hour)
	projection := f.ProjectMonthlyCost(context.Background(), "unknown-org")

	if projection != 0.0 {
		t.Errorf("Expected 0.0 for unknown org, got %f", projection)
	}
}

func TestForecaster_TrackEvent_Pruning(t *testing.T) {
	f := NewForecaster(1 * time.Hour)
	tenantID := "test-org-prune"

	now := time.Now()

	// Add an event outside the window
	f.events[tenantID] = []Event{
		{Timestamp: now.Add(-2 * time.Hour), Cost: 100.0},
	}

	// Track a new event (this should trigger pruning)
	f.TrackEvent(tenantID, 5.0)

	if len(f.events[tenantID]) != 1 {
		t.Errorf("Expected 1 event after pruning, got %d", len(f.events[tenantID]))
	}

	if f.events[tenantID][0].Cost != 5.0 {
		t.Errorf("Expected pruned event cost 5.0, got %f", f.events[tenantID][0].Cost)
	}
}
