package orchestration

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// TokenTracker defines the interface for tracking token usage
type TokenTracker interface {
	ActiveOrganizations(ctx context.Context) []string
	Summary(orgID string) TokenSummary
}

// TokenSummary contains the token usage summary for an organization
type TokenSummary struct {
	TotalTokens int64
}

// StartTokenBurnRateForecasting runs a background worker that calculates the token burn rate
func StartTokenBurnRateForecasting(ctx context.Context, tracker TokenTracker, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	// Store history of usage for calculating moving average (e.g. over the last 5 minutes)
	// Map of organizationID to a slice of totalTokens recorded each minute
	history := make(map[string][]int64)

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			orgIDs := tracker.ActiveOrganizations(ctx)
			for _, orgID := range orgIDs {
				summary := tracker.Summary(orgID)
				if summary.TotalTokens > 0 {
					h := history[orgID]
					h = append(h, summary.TotalTokens)

					// Keep only the last 5 data points for a 5-minute moving average
					if len(h) > 5 {
						h = h[1:]
					}
					history[orgID] = h

					if len(h) > 1 {
						// Calculate moving average burn rate (tokens per interval)
						rate := float64(h[len(h)-1] - h[0]) / float64(len(h)-1)
						telemetry.RecordTokenBurnRate(ctx, orgID, rate)
					}
				}
			}
		}
	}
}
