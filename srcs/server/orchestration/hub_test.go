package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/billing"
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
	ProcessForecastTick(ctx, history, tracker.ActiveOrganizations, func(orgID string) int64 { return tracker.Summary(orgID).TotalTokens })
	if calls != 1 {
		t.Fatalf("Expected tracker to be called 1 time, got %d", calls)
	}

	// Second tick
	ProcessForecastTick(ctx, history, tracker.ActiveOrganizations, func(orgID string) int64 { return tracker.Summary(orgID).TotalTokens })
	if calls != 2 {
		t.Fatalf("Expected tracker to be called 2 times, got %d", calls)
	}
}
