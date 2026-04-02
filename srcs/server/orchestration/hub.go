package orchestration

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// TokenTracker defines an interface to fetch organization tracking data.
type TokenTracker interface {
	ActiveOrganizations(ctx context.Context) []string
	Summary(organizationID string) billing.Summary
}

// StartTokenBurnForecaster starts a background worker that extrapolates token usage.
func StartTokenBurnForecaster(ctx context.Context, tracker TokenTracker) {
	// Expose ticker duration to allow overriding in tests.
	StartTokenBurnForecasterWithTicker(ctx, tracker, 1*time.Minute)
}

// StartTokenBurnForecasterWithTicker is the underlying implementation that accepts a custom tick duration.
func StartTokenBurnForecasterWithTicker(ctx context.Context, tracker TokenTracker, tickDuration time.Duration) {
	ticker := time.NewTicker(tickDuration)
	defer ticker.Stop()

	// Store history of usage for calculating moving average (e.g. over the last 5 ticks)
	// Map of organizationID to a slice of totalTokens recorded each tick
	history := make(map[string][]int64)

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if tracker == nil {
				continue
			}
			orgIDs := tracker.ActiveOrganizations(ctx)
			for _, orgID := range orgIDs {
				summary := tracker.Summary(orgID)
				if summary.TotalTokens > 0 {
					h := history[orgID]
					h = append(h, summary.TotalTokens)

					// Keep only the last 5 data points for a 5-tick moving average
					if len(h) > 5 {
						h = h[1:]
					}
					history[orgID] = h

					if len(h) > 1 {
						// Calculate moving average burn rate (tokens per tick)
						rate := float64(h[len(h)-1]-h[0]) / float64(len(h)-1)
						telemetry.RecordTokenBurnRate(ctx, orgID, rate)
					}
				}
			}
		}
	}
}
