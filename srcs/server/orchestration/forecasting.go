package orchestration

import (
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type TokenBurnRateEngine struct {
	mu            sync.Mutex
	tenantUsage   map[string][]tokenUsageRecord
	window        time.Duration
	forecastPeriod time.Duration
	ticker        *time.Ticker
	quit          chan struct{}
}

type tokenUsageRecord struct {
	timestamp time.Time
	count     int64
}

func NewTokenBurnRateEngine(window time.Duration, forecastPeriod time.Duration) *TokenBurnRateEngine {
	return &TokenBurnRateEngine{
		tenantUsage:   make(map[string][]tokenUsageRecord),
		window:        window,
		forecastPeriod: forecastPeriod,
		quit:          make(chan struct{}),
	}
}

func (e *TokenBurnRateEngine) Start() {
	e.ticker = time.NewTicker(1 * time.Minute)
	go func() {
		for {
			select {
			case <-e.ticker.C:
				e.calculateForecast()
			case <-e.quit:
				e.ticker.Stop()
				return
			}
		}
	}()
}

func (e *TokenBurnRateEngine) Stop() {
	close(e.quit)
}

func (e *TokenBurnRateEngine) RecordUsage(tenantID string, count int64) {
	e.mu.Lock()
	defer e.mu.Unlock()

	now := time.Now()
	e.tenantUsage[tenantID] = append(e.tenantUsage[tenantID], tokenUsageRecord{
		timestamp: now,
		count:     count,
	})

	// clean up old records
	cutoff := now.Add(-e.window)
	filtered := []tokenUsageRecord{}
	for _, record := range e.tenantUsage[tenantID] {
		if record.timestamp.After(cutoff) {
			filtered = append(filtered, record)
		}
	}
	e.tenantUsage[tenantID] = filtered
}

func (e *TokenBurnRateEngine) calculateForecast() {
	e.mu.Lock()
	defer e.mu.Unlock()

	now := time.Now()
	cutoff := now.Add(-e.window)

	for tenantID, records := range e.tenantUsage {
		var totalCount int64
		var oldest time.Time

		filtered := []tokenUsageRecord{}
		for _, record := range records {
			if record.timestamp.After(cutoff) {
				filtered = append(filtered, record)
				totalCount += record.count
				if oldest.IsZero() || record.timestamp.Before(oldest) {
					oldest = record.timestamp
				}
			}
		}
		e.tenantUsage[tenantID] = filtered

		if len(filtered) < 2 {
			continue // Not enough data
		}

		duration := now.Sub(oldest)
		if duration <= 0 {
			continue
		}

		ratePerNanosecond := float64(totalCount) / float64(duration)
		forecast := ratePerNanosecond * float64(e.forecastPeriod)

		telemetry.UpdateTokenBurnRateForecast(tenantID, forecast)
	}
}
