package orchestration

import (
	"context"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// BurnRateForecaster tracks token usage to extrapolate burn rates.
type BurnRateForecaster struct {
	mu           sync.Mutex
	usageHistory map[string][]usagePoint
	window       time.Duration
	interval     time.Duration
	cancel       context.CancelFunc
}

type usagePoint struct {
	timestamp time.Time
	tokens    float64
}

var globalForecaster *BurnRateForecaster

func init() {
	globalForecaster = &BurnRateForecaster{
		usageHistory: make(map[string][]usagePoint),
		window:       5 * time.Minute,
		interval:     1 * time.Minute,
	}
	// Note: We don't start the loop here to avoid unexpected background tasks during some init phases.
	// But in a real system we would start it when the Hub starts.
}

// StartForecaster starts the background routine to compute moving average token burn rate.
func StartForecaster(ctx context.Context) {
	ctx, cancel := context.WithCancel(ctx)
	globalForecaster.cancel = cancel

	go func() {
		ticker := time.NewTicker(globalForecaster.interval)
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				globalForecaster.computeAndExportBurnRate(context.Background())
			}
		}
	}()
}

// StopForecaster stops the background routine.
func StopForecaster() {
	if globalForecaster.cancel != nil {
		globalForecaster.cancel()
	}
}

// RecordUsage records token usage for a given tenant.
func RecordUsage(tenantID string, tokens float64) {
	globalForecaster.mu.Lock()
	defer globalForecaster.mu.Unlock()

	now := time.Now()
	globalForecaster.usageHistory[tenantID] = append(globalForecaster.usageHistory[tenantID], usagePoint{
		timestamp: now,
		tokens:    tokens,
	})

	// Cleanup old history
	cutoff := now.Add(-globalForecaster.window)
	filtered := []usagePoint{}
	for _, p := range globalForecaster.usageHistory[tenantID] {
		if p.timestamp.After(cutoff) {
			filtered = append(filtered, p)
		}
	}
	globalForecaster.usageHistory[tenantID] = filtered
}

func (f *BurnRateForecaster) computeAndExportBurnRate(ctx context.Context) {
	f.mu.Lock()
	defer f.mu.Unlock()

	now := time.Now()
	cutoff := now.Add(-f.window)

	for tenantID, points := range f.usageHistory {
		var total float64
		var count int
		for _, p := range points {
			if p.timestamp.After(cutoff) {
				total += p.tokens
				count++
			}
		}

		if count > 0 {
			// tokens per minute in the window
			// duration in minutes
			minutes := f.window.Minutes()
			rate := total / minutes

			// Expose predictive cost alert metric using the telemetry package
			telemetry.RecordTokenBurnRate(ctx, tenantID, rate)
		}
	}
}
