package orchestration

import (
	"context"
	"log/slog"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

var (
	burnRateMu            sync.Mutex
	tenantTokenUsage      = make(map[string]int64)
	tenantPreviousUsage   = make(map[string]int64)
	burnRateWorkerStarted bool
	burnRateWorkerCancel  context.CancelFunc
)

// RecordUsageForForecasting increments the local token counter for a specific organization/tenant
// to be used by the background token burn rate forecasting worker.
func RecordUsageForForecasting(organizationID string, tokens int64) {
	burnRateMu.Lock()
	defer burnRateMu.Unlock()
	tenantTokenUsage[organizationID] += tokens
}

// StartTokenBurnRateForecastingEngine starts the background daemon that
// calculates the token burn rate forecast based on a moving average.
func StartTokenBurnRateForecastingEngine(ctx context.Context) {
	burnRateMu.Lock()
	if burnRateWorkerStarted {
		burnRateMu.Unlock()
		return
	}
	burnRateWorkerStarted = true
	workerCtx, cancel := context.WithCancel(ctx)
	burnRateWorkerCancel = cancel
	burnRateMu.Unlock()

	go func() {
		ticker := time.NewTicker(1 * time.Minute)
		defer ticker.Stop()

		for {
			select {
			case <-workerCtx.Done():
				return
			case <-ticker.C:
				calculateBurnRate(ctx)
			}
		}
	}()
}

func calculateBurnRate(ctx context.Context) {
	burnRateMu.Lock()
	defer burnRateMu.Unlock()

	for orgID, currentUsage := range tenantTokenUsage {
		prevUsage := tenantPreviousUsage[orgID]
		diff := currentUsage - prevUsage

		// The forecast burn rate per minute.
		// Since this runs every 1 minute, the difference is the tokens burned in the last minute.
		// We could implement a more complex moving average, but the latest minute rate is a good start.
		rate := float64(diff)

		// Record to telemetry
		telemetry.RecordTokenBurnRate(ctx, orgID, rate)

		// Optional: we can add logging for budget alerts if the rate is too high.
		if rate > 10000 {
			slog.Warn("High token burn rate detected", "organization_id", orgID, "rate", rate)
		}

		tenantPreviousUsage[orgID] = currentUsage
	}
}

// ResetForecastingEngineForTest allows tests to reset the state.
func ResetForecastingEngineForTest() {
	burnRateMu.Lock()
	defer burnRateMu.Unlock()
	if burnRateWorkerCancel != nil {
		burnRateWorkerCancel()
		burnRateWorkerCancel = nil
	}
	tenantTokenUsage = make(map[string]int64)
	tenantPreviousUsage = make(map[string]int64)
	burnRateWorkerStarted = false
}
