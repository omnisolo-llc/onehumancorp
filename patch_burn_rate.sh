#!/bin/bash
cat << 'INNER_EOF' >> srcs/server/orchestration/service.go

// TokenTracker defines the interface needed for burn rate forecasting.
type TokenTracker interface {
	ActiveOrganizations(ctx context.Context) []string
	Summary(organizationID string) interface{ TotalTokens() int64 }
}

type summaryWrapper struct {
	tokens int64
}

func (s summaryWrapper) TotalTokens() int64 {
	return s.tokens
}

// RunTokenBurnRateForecasting starts a background worker that calculates and records the token burn rate forecast.
func RunTokenBurnRateForecasting(ctx context.Context, tracker interface {
	ActiveOrganizations(ctx context.Context) []string
	Summary(organizationID string) interface{ TotalTokens() int64 }
}) {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

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
				if summary.TotalTokens() > 0 {
					h := history[orgID]
					h = append(h, summary.TotalTokens())

					// Keep only the last 5 data points for a 5-minute moving average
					if len(h) > 5 {
						h = h[1:]
					}
					history[orgID] = h

					if len(h) > 1 {
						// Calculate moving average burn rate (tokens per minute)
						rate := float64(h[len(h)-1] - h[0]) / float64(len(h)-1)
						telemetry.RecordTokenBurnRate(ctx, orgID, rate)
					}
				}
			}
		}
	}
}
INNER_EOF
