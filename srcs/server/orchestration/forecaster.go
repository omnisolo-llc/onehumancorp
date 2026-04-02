package orchestration

import (
	"context"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// TokenForecaster continuously computes a 5-minute moving average of token burn rate per organisation.
type TokenForecaster struct {
	mu          sync.Mutex
	history     map[string][]tokenUsageEvent
	window      time.Duration
	interval    time.Duration
	stopCh      chan struct{}
	hub         *Hub
}

type tokenUsageEvent struct {
	timestamp time.Time
	count     int64
}

// NewTokenForecaster constructs a new TokenForecaster for the given hub.
func NewTokenForecaster(hub *Hub) *TokenForecaster {
	return &TokenForecaster{
		history:  make(map[string][]tokenUsageEvent),
		window:   5 * time.Minute,
		interval: 1 * time.Minute,
		stopCh:   make(chan struct{}),
		hub:      hub,
	}
}

// Start initiates the background moving average calculation loop.
func (f *TokenForecaster) Start() {
	telemetry.RecordTokenUsageCallback = f.recordUsage

	go f.loop()
}

// Stop terminates the background forecaster daemon.
func (f *TokenForecaster) Stop() {
	telemetry.RecordTokenUsageCallback = nil
	close(f.stopCh)
}

func (f *TokenForecaster) recordUsage(ctx context.Context, agentID string, role string, model string, tokenType string, count int64) {
	// Discover organisation ID associated with agent
	orgID := "default"
	if f.hub != nil {
		if agent, ok := f.hub.Agent(agentID); ok {
			if agent.OrganizationID != "" {
				orgID = agent.OrganizationID
			}
		}
	}

	f.mu.Lock()
	defer f.mu.Unlock()

	f.history[orgID] = append(f.history[orgID], tokenUsageEvent{
		timestamp: time.Now(),
		count:     count,
	})
}

func (f *TokenForecaster) loop() {
	ticker := time.NewTicker(f.interval)
	defer ticker.Stop()

	for {
		select {
		case <-f.stopCh:
			return
		case <-ticker.C:
			f.calculateAndEmit()
		}
	}
}

func (f *TokenForecaster) calculateAndEmit() {
	f.mu.Lock()
	defer f.mu.Unlock()

	now := time.Now()
	cutoff := now.Add(-f.window)

	for orgID, events := range f.history {
		var validEvents []tokenUsageEvent
		var totalTokens int64

		// Prune old events outside the window and sum recent usage
		for _, e := range events {
			if e.timestamp.After(cutoff) {
				validEvents = append(validEvents, e)
				totalTokens += e.count
			}
		}

		// Update history to only contain events within the time window
		if len(validEvents) > 0 {
			f.history[orgID] = validEvents
		} else {
			delete(f.history, orgID)
		}

		// Calculate rate: total tokens per minute
		// If totalTokens is X over 5 minutes, average per minute is X / 5.
		ratePerMinute := float64(totalTokens) / f.window.Minutes()
		telemetry.RecordTokenBurnRate(context.Background(), orgID, ratePerMinute)
	}
}
