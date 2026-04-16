package kairos

import (
	"github.com/prometheus/client_golang/prometheus"
)

var (
	TransitionsTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_kairos_transitions_total",
			Help: "Total number of KAIROS state transitions",
		},
		[]string{"mode", "status"},
	)

	TransitionDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name: "ohc_kairos_transition_duration_seconds",
			Help: "Latency of KAIROS state transitions in seconds",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"mode"},
	)
)

func init() {
	prometheus.MustRegister(TransitionsTotal)
	prometheus.MustRegister(TransitionDuration)
}
