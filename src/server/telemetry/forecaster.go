package telemetry

import (
	"context"
	"log/slog"
	"sync"
	"time"

	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

// Forecaster calculates the moving average token burn rate per tenant.
type Forecaster struct {
	mu           sync.Mutex
	usageHistory map[string][]tokenUsageRecord
	budgets      map[string]int64
	interval     time.Duration
	window       time.Duration
	stopCh       chan struct{}
}

type tokenUsageRecord struct {
	timestamp time.Time
	tokens    int64
}

// NewForecaster creates a new Forecaster.
func NewForecaster(interval time.Duration, window time.Duration) *Forecaster {
	return &Forecaster{
		usageHistory: make(map[string][]tokenUsageRecord),
		budgets:      make(map[string]int64),
		interval:     interval,
		window:       window,
		stopCh:       make(chan struct{}),
	}
}

// Start begins the background worker loop.
func (f *Forecaster) Start(ctx context.Context) {
	go func() {
		ticker := time.NewTicker(f.interval)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-f.stopCh:
				return
			case <-ticker.C:
				f.calculateAndRecordRates(ctx)
			}
		}
	}()
}

// Stop halts the background worker.
func (f *Forecaster) Stop() {
	f.mu.Lock()
	defer f.mu.Unlock()
	select {
	case <-f.stopCh:
		// already closed
	default:
		close(f.stopCh)
	}
}

// RecordUsage records a new token usage event for a tenant.
func (f *Forecaster) RecordUsage(organizationID string, tokens int64) {
	f.mu.Lock()
	defer f.mu.Unlock()

	now := time.Now()
	f.usageHistory[organizationID] = append(f.usageHistory[organizationID], tokenUsageRecord{
		timestamp: now,
		tokens:    tokens,
	})

	// Prune old records outside the window
	cutoff := now.Add(-f.window)
	var filtered []tokenUsageRecord
	for _, record := range f.usageHistory[organizationID] {
		if record.timestamp.After(cutoff) {
			filtered = append(filtered, record)
		}
	}
	f.usageHistory[organizationID] = filtered
}

// SetBudget sets the token budget for a tenant.
func (f *Forecaster) SetBudget(organizationID string, budget int64) {
	f.mu.Lock()
	defer f.mu.Unlock()
	f.budgets[organizationID] = budget
}

func (f *Forecaster) calculateAndRecordRates(ctx context.Context) {
	f.mu.Lock()
	defer f.mu.Unlock()

	now := time.Now()
	cutoff := now.Add(-f.window)

	for orgID, records := range f.usageHistory {
		var totalTokens int64
		var filtered []tokenUsageRecord

		for _, record := range records {
			if record.timestamp.After(cutoff) {
				filtered = append(filtered, record)
				totalTokens += record.tokens
			}
		}

		f.usageHistory[orgID] = filtered

		// Calculate rate per window (default 1h)
		windowDuration := f.window.Minutes()
		if windowDuration > 0 {
			// tokens per minute
			ratePerMin := float64(totalTokens) / windowDuration

			// Extrapolate to 24h
			prediction24h := ratePerMin * 60 * 24

			f.recordRates(ctx, orgID, ratePerMin, prediction24h)
			f.checkBudget(ctx, orgID, prediction24h)
		}
	}
}

func (f *Forecaster) recordRates(ctx context.Context, organizationID string, ratePerMin float64, prediction24h float64) {
	// Use existing helper if it exists, otherwise use the gauge directly
	RecordTokenBurnRate(ctx, organizationID, ratePerMin)

	if TokenBurnRatePredicted24h != nil {
		TokenBurnRatePredicted24h.Record(ctx, prediction24h, metric.WithAttributes(
			attribute.String("organization_id", organizationID),
		))
	}
}

func (f *Forecaster) checkBudget(ctx context.Context, organizationID string, prediction24h float64) {
	budget, ok := f.budgets[organizationID]
	if !ok || budget <= 0 {
		return
	}

	if prediction24h > float64(budget) {
		slog.WarnContext(ctx, "token budget overrun predicted",
			"organization_id", organizationID,
			"predicted_24h", prediction24h,
			"budget", budget,
		)

		if TokenBudgetAlertTotal != nil {
			TokenBudgetAlertTotal.Add(ctx, 1, metric.WithAttributes(
				attribute.String("organization_id", organizationID),
			))
		}
	}
}
