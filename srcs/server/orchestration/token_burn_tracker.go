package orchestration

import (
	"context"
	"log/slog"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type TokenBurnTracker struct {
	mu           sync.RWMutex
	interval     time.Duration
	windowSize   int
	stopCh       chan struct{}
	billingTrack *billing.Tracker
	history      map[string][]int64
	lastTokens   map[string]int64
	stopOnce     sync.Once
}

func NewTokenBurnTracker(interval time.Duration, windowSize int, billingTrack *billing.Tracker) *TokenBurnTracker {
	return &TokenBurnTracker{
		interval:     interval,
		windowSize:   windowSize,
		stopCh:       make(chan struct{}),
		billingTrack: billingTrack,
		history:      make(map[string][]int64),
		lastTokens:   make(map[string]int64),
	}
}

func (t *TokenBurnTracker) Start(ctx context.Context) {
	ticker := time.NewTicker(t.interval)
	go func() {
		for {
			select {
			case <-ctx.Done():
				ticker.Stop()
				return
			case <-t.stopCh:
				ticker.Stop()
				return
			case <-ticker.C:
				t.calculateAndEmit(ctx)
			}
		}
	}()
}

func (t *TokenBurnTracker) Stop() {
	t.stopOnce.Do(func() {
		close(t.stopCh)
	})
}

func (t *TokenBurnTracker) calculateAndEmit(ctx context.Context) {
	if t.billingTrack == nil {
		return
	}

	orgs := t.billingTrack.ActiveOrganizations(ctx)

	t.mu.Lock()
	defer t.mu.Unlock()

	for _, orgID := range orgs {
		summary := t.billingTrack.Summary(orgID)
		currentTotal := summary.TotalTokens

		lastTotal, exists := t.lastTokens[orgID]
		t.lastTokens[orgID] = currentTotal
		if !exists {
			// First time seeing this org, we don't know the delta over the interval
			// So we just record the baseline and skip emitting a diff for the first interval.
			continue
		}
		diff := currentTotal - lastTotal
		if diff < 0 {
			diff = 0
		}

		hist := t.history[orgID]
		if len(hist) >= t.windowSize {
			hist = hist[1:]
		}
		hist = append(hist, diff)
		t.history[orgID] = hist

		var sum int64
		for _, v := range hist {
			sum += v
		}

		avg := float64(sum) / float64(len(hist))
		rate := avg / t.interval.Minutes()

		telemetry.RecordTokenBurnRate(ctx, orgID, rate)
		slog.Debug("emitted token burn rate forecast", "org_id", orgID, "rate", rate)
	}
}
