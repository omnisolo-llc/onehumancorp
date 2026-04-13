package growth

import (
	"sync"
	"time"
)

// ExperimentEvent tracks a single user's assignment to an experiment variant.
type ExperimentEvent struct {
	ID        string    `json:"id"`
	UserID    string    `json:"userId"`
	Variant   string    `json:"variant"` // e.g., "local_first", "cloud_convenience"
	Converted bool      `json:"converted"`
	CreatedAt time.Time `json:"createdAt"`
}

// ExperimentTracker manages events for A/B testing.
type ExperimentTracker struct {
	mu     sync.RWMutex
	Events []ExperimentEvent
}

func NewExperimentTracker() *ExperimentTracker {
	return &ExperimentTracker{
		Events: make([]ExperimentEvent, 0),
	}
}

func (et *ExperimentTracker) TrackAssignment(userID string, variant string) {
	et.mu.Lock()
	defer et.mu.Unlock()
	et.Events = append(et.Events, ExperimentEvent{
		ID:        time.Now().UTC().Format("20060102150405"),
		UserID:    userID,
		Variant:   variant,
		Converted: false,
		CreatedAt: time.Now().UTC(),
	})
}

func (et *ExperimentTracker) MarkConverted(userID string) {
	et.mu.Lock()
	defer et.mu.Unlock()
	for i := range et.Events {
		if et.Events[i].UserID == userID && !et.Events[i].Converted {
			et.Events[i].Converted = true
			return
		}
	}
}

// ExperimentMetrics holds aggregated stats for a variant.
type ExperimentMetrics struct {
	Variant        string  `json:"variant"`
	TotalAssigned  int     `json:"totalAssigned"`
	TotalConverted int     `json:"totalConverted"`
	ConversionRate float64 `json:"conversionRate"`
}

func (et *ExperimentTracker) GetMetrics() map[string]ExperimentMetrics {
	et.mu.RLock()
	defer et.mu.RUnlock()

	stats := make(map[string]*ExperimentMetrics)

	for _, event := range et.Events {
		if _, exists := stats[event.Variant]; !exists {
			stats[event.Variant] = &ExperimentMetrics{Variant: event.Variant}
		}
		stats[event.Variant].TotalAssigned++
		if event.Converted {
			stats[event.Variant].TotalConverted++
		}
	}

	result := make(map[string]ExperimentMetrics)
	for variant, metrics := range stats {
        if metrics.TotalAssigned > 0 {
            metrics.ConversionRate = (float64(metrics.TotalConverted) / float64(metrics.TotalAssigned)) * 100.0
        }
		result[variant] = *metrics
	}

	return result
}
