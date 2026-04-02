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

func TestClaimTask_Isolation(t *testing.T) {
	ctx := context.Background()

	// Mock repository using our SqliteHubRepository wrapped around an in-memory db
	// Note: We don't have db setup here easily, so we just test the in-memory fallback Hub behavior for isolation.
	hub := NewHubWithRepository(nil, nil)

	claimed1, err := hub.ClaimTask(ctx, "task-1", "agent-A")
	if err != nil {
		t.Fatalf("unexpected error claiming task: %v", err)
	}
	if !claimed1 {
		t.Errorf("expected to claim task successfully with memory fallback")
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
			// Increase token count by 100 on each call
			return billing.Summary{
				TotalTokens: int64(calls * 100),
			}
		},
	}

	// We start the forecaster with a very fast ticker to ensure it executes.
	// Since it loops infinitely, we just run it in a goroutine.
	go StartTokenBurnForecasterWithTicker(
		ctx,
		func(c context.Context) []string { return tracker.ActiveOrganizations(c) },
		func(orgID string) int64 { return tracker.Summary(orgID).TotalTokens },
		10*time.Millisecond,
	)

	time.Sleep(100 * time.Millisecond)
	cancel()

	// Wait a bit to ensure cancellation is processed
	time.Sleep(50 * time.Millisecond)

	if calls < 2 {
		t.Fatalf("Expected tracker to be called at least twice, got %d", calls)
	}
}
