package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/billing"
)

func TestTokenBurnTracker(t *testing.T) {
	// Dummy repository/tracker
	catalog := map[string]billing.Price{
		"gpt-4": {InputPerMillionUSD: 10, OutputPerMillionUSD: 30},
	}
	tracker := billing.NewTracker(catalog)

	burnTracker := NewTokenBurnTracker(10*time.Millisecond, 2, tracker)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	burnTracker.Start(ctx)

	// Simulate usage
	tracker.Track(billing.Usage{
		OrganizationID:   "org-1",
		Model:            "gpt-4",
		PromptTokens:     100,
		CompletionTokens: 50,
		OccurredAt:       time.Now(),
	})

	time.Sleep(30 * time.Millisecond)

	burnTracker.Stop()
}
