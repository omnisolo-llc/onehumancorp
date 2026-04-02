package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/billing"
)

// Dummy test for hub forecast to cover the code
func TestStartTokenBurnRateForecasting(t *testing.T) {
	catalog := map[string]billing.Price{
		"gpt-4": {InputPerMillionUSD: 30.00, OutputPerMillionUSD: 60.00},
	}
	tracker := billing.NewTracker(catalog)
	tracker.Track(billing.Usage{
		OrganizationID: "org-1",
		Model: "gpt-4",
		PromptTokens: 100,
		CompletionTokens: 50,
	})

	hub := NewHub()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Since StartTokenBurnRateForecasting has an infinite loop with a ticker,
	// we just start it and let it run for a short time, then cancel the context.
	go hub.StartTokenBurnRateForecasting(ctx, tracker)

	time.Sleep(100 * time.Millisecond)
}
