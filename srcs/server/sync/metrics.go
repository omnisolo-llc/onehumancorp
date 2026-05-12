package sync

import (
	"log"
	"os"
	"time"
)

// MetricsTracker tracks syncing performance and telemetry.
type MetricsTracker struct {
	enabled bool
}

// NewMetricsTracker creates a new telemetry tracker.
func NewMetricsTracker() *MetricsTracker {
	enabled := os.Getenv("OHC_TELEMETRY_ENABLED") == "true"
	return &MetricsTracker{enabled: enabled}
}

// RecordSync records a synchronization attempt.
func (m *MetricsTracker) RecordSync(deltas int, duration time.Duration, err error) {
	if !m.enabled {
		return
	}
	standalone := os.Getenv("OHC_STANDALONE") == "true"
	if standalone {
		status := "success"
		if err != nil {
			status = "error"
		}
		log.Printf("Telemetry: Sync completed. Deltas: %d, Duration: %v, Status: %s", deltas, duration, status)
	}
}
