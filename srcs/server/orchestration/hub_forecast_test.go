package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type mockTracker struct {
	orgs    []string
	summary int64
}

func (m *mockTracker) ActiveOrganizations(ctx context.Context) []string {
	return m.orgs
}

func (m *mockTracker) Summary(orgID string) interface{ TotalTokens() int64 } {
	return summaryWrapper{tokens: m.summary}
}

func TestRunTokenBurnRateForecasting(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	tracker := &mockTracker{orgs: []string{"org1"}, summary: 100}

	go RunTokenBurnRateForecasting(ctx, tracker)

	time.Sleep(10 * time.Millisecond) // Let it spin up (ticker won't fire in this test since it's 1m, but we ensure it runs without panicking)
	cancel()
}
