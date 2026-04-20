package telemetry

import (
	"context"
	"sync"
	"time"
)

type TokenForecastWorker struct {
	mu           sync.Mutex
	usageHistory map[string]float64 // Stores the current EWMA rate per tenant
	interval     time.Duration
	alpha        float64
	currentUsage map[string]int64
	stopCh       chan struct{}
}

func NewTokenForecastWorker(interval time.Duration, alpha float64) *TokenForecastWorker {
	return &TokenForecastWorker{
		usageHistory: make(map[string]float64),
		currentUsage: make(map[string]int64),
		interval:     interval,
		alpha:        alpha,
		stopCh:       make(chan struct{}),
	}
}

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

func (w *TokenForecastWorker) Stop() {
	close(w.stopCh)
}

func (w *TokenForecastWorker) RecordUsage(organizationID string, tokens int64) {
	w.mu.Lock()
	defer w.mu.Unlock()
	w.currentUsage[organizationID] += tokens
}

func (w *TokenForecastWorker) calculateAndRecordRates(ctx context.Context) {
	w.mu.Lock()
	defer w.mu.Unlock()

	intervalMinutes := w.interval.Minutes()
	if intervalMinutes <= 0 {
		return
	}

	for orgID, tokens := range w.currentUsage {
		currentRate := float64(tokens) / intervalMinutes
		oldEWMARate, exists := w.usageHistory[orgID]
		var newEWMARate float64
		if !exists {
			newEWMARate = currentRate
		} else {
			newEWMARate = (w.alpha * currentRate) + ((1.0 - w.alpha) * oldEWMARate)
		}

		w.usageHistory[orgID] = newEWMARate
		w.currentUsage[orgID] = 0 // Reset current usage

		RecordTokenBurnRate(ctx, orgID, newEWMARate)
	}
}
