package orchestration

import (
	"context"
	"log/slog"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type tokenBurnRateEngine struct {
	mu               sync.Mutex
	lastUsage        int64
	usageHistory     []float64
	historySize      int
	pollInterval     time.Duration
	usageTrackerFunc func() int64
	stopChan         chan struct{}
}

func newTokenBurnRateEngine(pollInterval time.Duration, usageTrackerFunc func() int64) *tokenBurnRateEngine {
	return &tokenBurnRateEngine{
		usageHistory:     make([]float64, 0),
		historySize:      5, // Calculate moving average over last 5 intervals
		pollInterval:     pollInterval,
		usageTrackerFunc: usageTrackerFunc,
		stopChan:         make(chan struct{}),
	}
}

func (e *tokenBurnRateEngine) start(ctx context.Context) {
	ticker := time.NewTicker(e.pollInterval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-e.stopChan:
			return
		case <-ticker.C:
			e.calculateAndEmit(ctx)
		}
	}
}

func (e *tokenBurnRateEngine) stop() {
	close(e.stopChan)
}

func (e *tokenBurnRateEngine) calculateAndEmit(ctx context.Context) {
	e.mu.Lock()
	defer e.mu.Unlock()

	currentUsage := e.usageTrackerFunc()
	delta := currentUsage - e.lastUsage
	e.lastUsage = currentUsage

	ratePerMinute := float64(delta) / e.pollInterval.Minutes()

	if len(e.usageHistory) >= e.historySize {
		e.usageHistory = e.usageHistory[1:]
	}
	e.usageHistory = append(e.usageHistory, ratePerMinute)

	var total float64
	for _, rate := range e.usageHistory {
		total += rate
	}
	movingAverage := total / float64(len(e.usageHistory))

	// Extrapolate usage and emit metrics
	telemetry.RecordTokenBurnRate(ctx, "default", movingAverage)

	// Emit predictive cost alerts if burn rate is suspiciously high (e.g. > 1000 tokens/min)
	if movingAverage > 1000 {
		slog.Warn("High Token Burn Rate Predicted", "moving_average_per_minute", movingAverage)
	}
}
