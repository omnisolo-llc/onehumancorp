package billing

import (
	"context"
	"log/slog"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// usageRecord stores a point-in-time snapshot of usage for an organization.
type usageRecord struct {
	timestamp time.Time
	tokens    int64
	costUSD   float64
}

// Forecaster implements a predictive engine for token and USD burn rates.
type Forecaster struct {
	mu           sync.RWMutex
	usageHistory map[string][]usageRecord
	interval     time.Duration
	window       time.Duration
	budgetLimit  float64 // Default monthly budget limit fallback
	orgBudgets   sync.Map // Map of organizationID -> float64 budget limit

	getData      func(ctx context.Context, orgID string) (tokens int64, costUSD float64)
	getActive    func(ctx context.Context) []string

	stopCh       chan struct{}
	stopOnce     sync.Once
}

// NewForecaster creates a new Forecaster.
func NewForecaster(interval, window time.Duration, budgetLimit float64) *Forecaster {
	return &Forecaster{
		usageHistory: make(map[string][]usageRecord),
		interval:     interval,
		window:       window,
		budgetLimit:  budgetLimit,
		stopCh:       make(chan struct{}),
	}
}

// SetDataProviders sets the functions used to fetch usage data.
func (f *Forecaster) SetDataProviders(getActive func(context.Context) []string, getData func(context.Context, string) (int64, float64)) {
	f.getActive = getActive
	f.getData = getData
}

// SetOrgBudget sets a specific monthly budget limit for an organization.
func (f *Forecaster) SetOrgBudget(orgID string, limit float64) {
	f.orgBudgets.Store(orgID, limit)
}

// Start begins the background forecasting loop.
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
				f.collectAndCalculate(ctx)
			}
		}
	}()
}

// Stop halts the background forecasting loop.
func (f *Forecaster) Stop() {
	f.stopOnce.Do(func() {
		close(f.stopCh)
	})
}

// collectAndCalculate gathers new data points and computes rates.
func (f *Forecaster) collectAndCalculate(ctx context.Context) {
	if f.getActive == nil || f.getData == nil {
		return
	}

	orgs := f.getActive(ctx)
	now := time.Now()
	cutoff := now.Add(-f.window)

	for _, orgID := range orgs {
		tokens, costUSD := f.getData(ctx, orgID)

		f.mu.Lock()
		history := f.usageHistory[orgID]
		history = append(history, usageRecord{
			timestamp: now,
			tokens:    tokens,
			costUSD:   costUSD,
		})

		// Prune old records
		var filtered []usageRecord
		for _, r := range history {
			if r.timestamp.After(cutoff) {
				filtered = append(filtered, r)
			}
		}
		f.usageHistory[orgID] = filtered

		// If we have enough data, calculate
		if len(filtered) >= 2 {
			first := filtered[0]
			last := filtered[len(filtered)-1]
			duration := last.timestamp.Sub(first.timestamp)

			if duration > 0 {
				tokenRate := float64(last.tokens-first.tokens) / duration.Minutes()
				usdRate := (last.costUSD - first.costUSD) / duration.Minutes()

				// Release lock before telemetry/logging to minimize critical section
				f.mu.Unlock()

				telemetry.RecordTokenBurnRate(ctx, orgID, tokenRate)
				telemetry.RecordUSDBurnRate(ctx, orgID, usdRate)

				f.checkBudget(orgID, usdRate)
				continue // Already unlocked
			}
		}
		f.mu.Unlock()
	}
}

func (f *Forecaster) checkBudget(orgID string, usdRate float64) {
	limit := f.budgetLimit
	if val, ok := f.orgBudgets.Load(orgID); ok {
		limit = val.(float64)
	}

	if limit > 0 {
		projectedMonthly := usdRate * 60 * 24 * 30
		if projectedMonthly > limit {
			slog.Warn("💰 Budget Overrun Predicted",
				"organization_id", orgID,
				"projected_monthly_usd", projectedMonthly,
				"budget_limit", limit,
				"usd_burn_rate_per_min", usdRate)
		}
	}
}

// GetBurnRates returns the current moving average burn rates for tokens and USD per minute.
func (f *Forecaster) GetBurnRates(orgID string) (tokensPerMin, usdPerMin float64) {
	f.mu.RLock()
	defer f.mu.RUnlock()

	history, ok := f.usageHistory[orgID]
	if !ok || len(history) < 2 {
		return 0, 0
	}

	first := history[0]
	last := history[len(history)-1]
	duration := last.timestamp.Sub(first.timestamp)

	if duration <= 0 {
		return 0, 0
	}

	tokensPerMin = float64(last.tokens-first.tokens) / duration.Minutes()
	usdPerMin = (last.costUSD - first.costUSD) / duration.Minutes()
	return
}
