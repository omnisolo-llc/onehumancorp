package telemetry

import (
	"context"
	"log"
	"sync"
	"time"
)

var GlobalTokenForecaster *TokenForecaster

func InitGlobalTokenForecaster(syncEngine *TelemetrySyncEngine) {
	GlobalTokenForecaster = NewTokenForecaster(syncEngine)
}

// TokenForecaster calculates the EWMA of token burn rate (tokens/second)
type TokenForecaster struct {
	mu            sync.RWMutex
	lastUsage     map[string]float64
	lastTimestamp map[string]time.Time
	burnRateEWMA  map[string]float64
	emaWeight     float64
	syncEngine    *TelemetrySyncEngine
}

func NewTokenForecaster(syncEngine *TelemetrySyncEngine) *TokenForecaster {
	return &TokenForecaster{
		lastUsage:     make(map[string]float64),
		lastTimestamp: make(map[string]time.Time),
		burnRateEWMA:  make(map[string]float64),
		emaWeight:     0.2, // 20% weight on new data
		syncEngine:    syncEngine,
	}
}

func (tf *TokenForecaster) RecordUsage(tenantID string, tokens float64) {
	tf.mu.Lock()
	defer tf.mu.Unlock()

	now := time.Now()

	lastTime, hasLastTime := tf.lastTimestamp[tenantID]
	if hasLastTime {
		duration := now.Sub(lastTime).Seconds()
		if duration > 0 {
			// tokens per second
			burnRate := tokens / duration

			if existingEWMA, hasEWMA := tf.burnRateEWMA[tenantID]; hasEWMA {
				tf.burnRateEWMA[tenantID] = (burnRate * tf.emaWeight) + (existingEWMA * (1.0 - tf.emaWeight))
			} else {
				tf.burnRateEWMA[tenantID] = burnRate
			}
		}
	} else {
		// Initialize without a burn rate (need 2 points to calculate rate)
		// But we'll set the timestamp and last usage below
	}

	tf.lastUsage[tenantID] = tokens
	tf.lastTimestamp[tenantID] = now
}

func (tf *TokenForecaster) CalculateEWMA(tenantID string) float64 {
	tf.mu.RLock()
	defer tf.mu.RUnlock()

	return tf.burnRateEWMA[tenantID]
}

func (tf *TokenForecaster) StartForecastingDaemon(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			tf.mu.RLock()
			var tenants []string
			for tenantID := range tf.burnRateEWMA {
				tenants = append(tenants, tenantID)
			}
			tf.mu.RUnlock()

			for _, tenantID := range tenants {
				ewma := tf.CalculateEWMA(tenantID)

				attrs := map[string]interface{}{
					"tenant_id": tenantID,
				}

				if tf.syncEngine != nil {
					err := tf.syncEngine.BufferMetric(ctx, "ohc_token_burn_rate_forecast", ewma, attrs)
					if err != nil {
						log.Printf("Failed to buffer token burn rate forecast for %s: %v", tenantID, err)
					}
				}
			}
		}
	}
}
