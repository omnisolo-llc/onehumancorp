package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/billing"
)

type mockTokenTracker struct {
	orgs        []string
	summaryFunc func(orgID string) billing.Summary
}

func (m *mockTokenTracker) ActiveOrganizations(ctx context.Context) []string {
	return m.orgs
}

func (m *mockTokenTracker) Summary(organizationID string) billing.Summary {
	if m.summaryFunc != nil {
		return m.summaryFunc(organizationID)
	}
	return billing.Summary{}
}

func TestProcessForecastTick(t *testing.T) {
	ctx := context.Background()

	calls := 0
	tracker := &mockTokenTracker{
		orgs: []string{"org1"},
		summaryFunc: func(orgID string) billing.Summary {
			calls++
			// Increase token count by 100 on each call
			return billing.Summary{
				TotalTokens: int64(calls * 100),
			}
		},
	}

	history := make(map[string][]int64)

	// First tick
	ProcessForecastTick(ctx, history, tracker.ActiveOrganizations, func(orgID string) int64 { return tracker.Summary(orgID).TotalTokens }, time.Minute)
	if calls != 1 {
		t.Fatalf("Expected tracker to be called 1 time, got %d", calls)
	}

	// Second tick
	ProcessForecastTick(ctx, history, tracker.ActiveOrganizations, func(orgID string) int64 { return tracker.Summary(orgID).TotalTokens }, time.Minute)
	if calls != 2 {
		t.Fatalf("Expected tracker to be called 2 times, got %d", calls)
	}

	// Third tick, make org inactive to test cleanup
	tracker.orgs = []string{}
	ProcessForecastTick(ctx, history, tracker.ActiveOrganizations, func(orgID string) int64 { return tracker.Summary(orgID).TotalTokens }, time.Minute)
	if _, ok := history["org1"]; ok {
		t.Fatalf("Expected org1 to be removed from history when inactive")
	}

	// Test nil callbacks
	ProcessForecastTick(ctx, history, nil, nil, time.Minute)
}
