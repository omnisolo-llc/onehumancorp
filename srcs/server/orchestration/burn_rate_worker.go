package orchestration

import (
	"context"
	"time"
	"sync"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type BurnRateEngine struct {
	mu           sync.Mutex
	usageHistory []int64
	running      bool
	stopChan     chan struct{}
}

var GlobalBurnRateEngine *BurnRateEngine
var burnRateEngineOnce sync.Once

func InitBurnRateEngine() {
	burnRateEngineOnce.Do(func() {
		GlobalBurnRateEngine = &BurnRateEngine{
			usageHistory: make([]int64, 0),
		}
		telemetry.RecordTokenUsageCallback = GlobalBurnRateEngine.TrackUsage
		GlobalBurnRateEngine.Start()
	})
}

func (e *BurnRateEngine) TrackUsage(ctx context.Context, orgID string, count int64) {
	e.mu.Lock()
	defer e.mu.Unlock()

	// Add the current count to the most recent bucket (we will accumulate in a minute window)
	if len(e.usageHistory) == 0 {
		e.usageHistory = append(e.usageHistory, count)
	} else {
		e.usageHistory[len(e.usageHistory)-1] += count
	}
}

func (e *BurnRateEngine) Start() {
	e.mu.Lock()
	if e.running {
		e.mu.Unlock()
		return
	}
	e.running = true
	e.stopChan = make(chan struct{})
	e.mu.Unlock()

	go e.run()
}

func (e *BurnRateEngine) Stop() {
	e.mu.Lock()
	defer e.mu.Unlock()
	if !e.running {
		return
	}
	close(e.stopChan)
	e.running = false
}

func (e *BurnRateEngine) run() {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ticker.C:
			e.calculateForecast()
		case <-e.stopChan:
			return
		}
	}
}

func (e *BurnRateEngine) calculateForecast() {
	e.mu.Lock()
	defer e.mu.Unlock()

	// Calculate the moving average over the recorded history
	var total int64
	for _, usage := range e.usageHistory {
		total += usage
	}

	var average float64
	if len(e.usageHistory) > 0 {
		average = float64(total) / float64(len(e.usageHistory))
	}

	// Update the Prometheus gauge
	telemetry.RecordTokenBurnRate(context.Background(), "default-org", average)

	// Append a new bucket for the next minute
	e.usageHistory = append(e.usageHistory, 0)

	// Keep history bounded, e.g., to last 60 minutes
	if len(e.usageHistory) > 60 {
		e.usageHistory = e.usageHistory[1:]
	}
}
