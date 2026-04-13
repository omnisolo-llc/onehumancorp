package orchestration

import (
	"context"
	"testing"
	"time"

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

func TestStartTokenBurnForecasterWithTicker(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	calls := 0
	tracker := &mockTokenTracker{
		orgs: []string{"org1"},
		summaryFunc: func(orgID string) billing.Summary {
			calls++
			if calls >= 2 {
				cancel() // Stop the loop
			}
			return billing.Summary{
				TotalTokens: int64(calls * 100),
			}
		},
	}

	StartTokenBurnForecasterWithTicker(
		ctx,
		tracker.ActiveOrganizations,
		func(orgID string) int64 { return tracker.Summary(orgID).TotalTokens },
		time.Millisecond*10,
	)

	if calls < 2 {
		t.Fatalf("Expected tracker to be called at least 2 times, got %d", calls)
	}
}

func TestStartTokenBurnForecaster(t *testing.T) {
	// Just verify it doesn't panic when we pass a canceled context immediately.
	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	StartTokenBurnForecaster(ctx, nil, nil)
}
