package orchestration

import (
	"context"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// TokenBurnForecaster calculates the moving average token burn rate per tenant.
type TokenBurnForecaster struct {
	mu          sync.Mutex
	usage       map[string]int64 // map[organizationID]int64
	windowStart time.Time
	hub         *Hub
	ctx         context.Context
	cancel      context.CancelFunc
	callbackID  int
}

// NewTokenBurnForecaster creates a new instance of TokenBurnForecaster.
func NewTokenBurnForecaster(hub *Hub) *TokenBurnForecaster {
	ctx, cancel := context.WithCancel(context.Background())
	f := &TokenBurnForecaster{
		usage:       make(map[string]int64),
		windowStart: time.Now(),
		hub:         hub,
		ctx:         ctx,
		cancel:      cancel,
	}

	// Register callback
	f.callbackID = telemetry.RegisterTokenUsageCallback(f.recordUsage)

	go f.runForecastLoop()

	return f
}

// Stop halts the forecasting loop
func (f *TokenBurnForecaster) Stop() {
	telemetry.DeregisterTokenUsageCallback(f.callbackID)
	f.cancel()
}

// ProcessForecastTick forcefully executes the internal logic of a single loop tick,
// usually invoked by the time ticker but exposed for comprehensive testing coverage.
func (f *TokenBurnForecaster) ProcessForecastTick() {
	f.mu.Lock()
	defer f.mu.Unlock()

	now := time.Now()
	elapsedMinutes := now.Sub(f.windowStart).Minutes()
	if elapsedMinutes <= 0 {
		elapsedMinutes = 1
	}

	for orgID, count := range f.usage {
		rate := float64(count) / elapsedMinutes
		// Record the forecast using telemetry
		telemetry.RecordTokenBurnRate(context.Background(), orgID, rate)
		// Reset count for next moving average window
		f.usage[orgID] = 0
	}

	f.windowStart = now
}

func (f *TokenBurnForecaster) recordUsage(ctx context.Context, agentID, role, model, tokenType string, count int64) {
	if f.hub == nil {
		return
	}
	agent, ok := f.hub.Agent(agentID)
	if !ok {
		return
	}

	f.mu.Lock()
	defer f.mu.Unlock()
	f.usage[agent.OrganizationID] += count
}

func (f *TokenBurnForecaster) runForecastLoop() {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-f.ctx.Done():
			return
		case <-ticker.C:
			f.ProcessForecastTick()
		}
	}
}
