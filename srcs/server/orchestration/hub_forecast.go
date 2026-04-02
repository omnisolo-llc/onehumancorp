package orchestration

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/onehumancorp/mono/srcs/server/billing"
)

// StartTokenBurnRateForecasting Engine calculates token burn rate forecast.
func (h *Hub) StartTokenBurnRateForecasting(ctx context.Context, tracker *billing.Tracker) {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	// Previous usage history per org
	history := make(map[string][]int64)

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			orgs := tracker.ActiveOrganizations(ctx)
			for _, org := range orgs {
				summary := tracker.Summary(org)

				// Keep track of the last few token counts to calculate a moving average
				if _, ok := history[org]; !ok {
					history[org] = make([]int64, 0)
				}

				history[org] = append(history[org], summary.TotalTokens)
				if len(history[org]) > 5 {
					history[org] = history[org][1:]
				}

				if len(history[org]) >= 2 {
					// Use telescoping sum optimization
					totalDiff := history[org][len(history[org])-1] - history[org][0]
					avgBurnRate := float64(totalDiff) / float64(len(history[org])-1)

					// Record the forecast
					telemetry.RecordTokenBurnRate(ctx, org, avgBurnRate)
				}
			}
		}
	}
}
