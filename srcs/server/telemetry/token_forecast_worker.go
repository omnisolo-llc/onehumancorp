package telemetry

import (
	"context"
	"sync"
	"time"
)

// TokenForecastWorker calculates the moving average token burn rate per tenant.
type TokenForecastWorker struct {
	mu           sync.Mutex
	usageHistory map[string][]tokenUsageRecord
	interval     time.Duration
	window       time.Duration
	stopCh       chan struct{}
}

type tokenUsageRecord struct {
	timestamp time.Time
	tokens    int64
}

// NewTokenForecastWorker creates a new TokenForecastWorker.
func NewTokenForecastWorker(interval time.Duration, window time.Duration) *TokenForecastWorker {
	return &TokenForecastWorker{
		usageHistory: make(map[string][]tokenUsageRecord),
		interval:     interval,
		window:       window,
		stopCh:       make(chan struct{}),
	}
}

// Start begins the background worker loop.
func (w *TokenForecastWorker) Start(ctx context.Context) {
	go func() {
		ticker := time.NewTicker(w.interval)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-w.stopCh:
				return
			case <-ticker.C:
				w.calculateAndRecordRates(ctx)
			}
		}
	}()
}

// Stop halts the background worker.
func (w *TokenForecastWorker) Stop() {
	close(w.stopCh)
}

// RecordUsage records a new token usage event for a tenant.
// In a real application, this might be hooked up to where tokens are actually counted,
// or the worker might query a database. For this implementation, we'll provide a way
// to feed data into it.
func (w *TokenForecastWorker) RecordUsage(organizationID string, tokens int64) {
	w.mu.Lock()
	defer w.mu.Unlock()

	now := time.Now()
	w.usageHistory[organizationID] = append(w.usageHistory[organizationID], tokenUsageRecord{
		timestamp: now,
		tokens:    tokens,
	})

	// Prune old records outside the window
	cutoff := now.Add(-w.window)
	var filtered []tokenUsageRecord
	for _, record := range w.usageHistory[organizationID] {
		if record.timestamp.After(cutoff) {
			filtered = append(filtered, record)
		}
	}
	w.usageHistory[organizationID] = filtered
}

func (w *TokenForecastWorker) calculateAndRecordRates(ctx context.Context) {
	w.mu.Lock()
	defer w.mu.Unlock()

	now := time.Now()
	cutoff := now.Add(-w.window)

	for orgID, records := range w.usageHistory {
		var totalTokens int64
		var filtered []tokenUsageRecord

		for _, record := range records {
			if record.timestamp.After(cutoff) {
				filtered = append(filtered, record)
				totalTokens += record.tokens
			}
		}

		w.usageHistory[orgID] = filtered

		// Calculate rate per minute
		windowMinutes := w.window.Minutes()
		if windowMinutes > 0 {
			rate := float64(totalTokens) / windowMinutes
			RecordTokenBurnRate(ctx, orgID, rate)
		}
	}
}
