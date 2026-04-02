package orchestration

import (
	"context"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// TokenBurnRateForecaster tracks token usage over time to calculate a moving average burn rate.
type TokenBurnRateForecaster struct {
	mu           sync.Mutex
	tenantTokens map[string][]tokenUsageRecord
	window       time.Duration
	ticker       *time.Ticker
	done         chan struct{}
}

type tokenUsageRecord struct {
	timestamp time.Time
	count     int64
}

// NewTokenBurnRateForecaster creates a new forecasting engine.
func NewTokenBurnRateForecaster(window time.Duration, updateInterval time.Duration) *TokenBurnRateForecaster {
	if window == 0 {
		window = 5 * time.Minute
	}
	if updateInterval == 0 {
		updateInterval = 1 * time.Minute
	}

	f := &TokenBurnRateForecaster{
		tenantTokens: make(map[string][]tokenUsageRecord),
		window:       window,
		ticker:       time.NewTicker(updateInterval),
		done:         make(chan struct{}),
	}
	return f
}

// Start begins the background worker to calculate and emit burn rates.
func (f *TokenBurnRateForecaster) Start() {
	go f.worker()
}

// Stop halts the background worker.
func (f *TokenBurnRateForecaster) Stop() {
	f.ticker.Stop()
	close(f.done)
}

// RecordUsage adds a new token usage count for a tenant.
func (f *TokenBurnRateForecaster) RecordUsage(tenantID string, count int64) {
	f.mu.Lock()
	defer f.mu.Unlock()

	now := time.Now()
	f.tenantTokens[tenantID] = append(f.tenantTokens[tenantID], tokenUsageRecord{
		timestamp: now,
		count:     count,
	})
}

func (f *TokenBurnRateForecaster) worker() {
	for {
		select {
		case <-f.ticker.C:
			f.calculateAndEmit()
		case <-f.done:
			return
		}
	}
}

func (f *TokenBurnRateForecaster) calculateAndEmit() {
	f.mu.Lock()
	defer f.mu.Unlock()

	now := time.Now()
	cutoff := now.Add(-f.window)

	for tenantID, records := range f.tenantTokens {
		var validRecords []tokenUsageRecord
		var totalTokens int64

		for _, r := range records {
			if r.timestamp.After(cutoff) {
				validRecords = append(validRecords, r)
				totalTokens += r.count
			}
		}

		f.tenantTokens[tenantID] = validRecords

		// Calculate moving average burn rate (tokens per minute)
		windowMinutes := f.window.Minutes()
		if windowMinutes > 0 {
			burnRate := float64(totalTokens) / windowMinutes
			telemetry.RecordTokenBurnRate(context.Background(), tenantID, burnRate)
		}
	}
}
