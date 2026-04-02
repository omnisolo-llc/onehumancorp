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

func TestHubPublishTask(t *testing.T) {
	hub := NewHub()

	task := &SharedTask{
		ID:        "task-1",
		MissionID: "mission-1",
		Title:     "Test Task",
	}

	// Should not panic when centrifuge node is nil
	err := hub.PublishTask(task)
	if err != nil {
		t.Fatalf("PublishTask returned error when centrifuge node was nil: %v", err)
	}
}
