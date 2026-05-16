package autodream

import (
	"github.com/prometheus/client_golang/prometheus"
)

var (
	MemoriesProcessedTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_autodream_memories_processed_total",
			Help: "Total number of memories processed by AutoDream",
		},
		[]string{"mode"},
	)

	BatchProcessingDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "ohc_autodream_batch_processing_duration_seconds",
			Help:    "Duration of AutoDream batch processing in seconds",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"mode"},
	)

	ConsolidationErrorsTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_autodream_consolidation_errors_total",
			Help: "Total number of AutoDream consolidation errors",
		},
		[]string{"mode", "error_type"},
	)
)

func init() {
	prometheus.MustRegister(MemoriesProcessedTotal)
	prometheus.MustRegister(BatchProcessingDuration)
	prometheus.MustRegister(ConsolidationErrorsTotal)
}
