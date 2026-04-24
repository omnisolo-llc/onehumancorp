package autodream

import (
	"github.com/prometheus/client_golang/prometheus"
)

var (
	// MemoriesProcessedTotal tracks the total number of memories processed by AutoDream pipelines.
	MemoriesProcessedTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_autodream_memories_processed_total",
			Help: "Total number of memories processed by the AutoDream consolidation pipeline.",
		},
		[]string{"mode", "source_type", "status"}, // mode: cloud/standalone, source_type: task/session/mission, status: success/failure
	)

	// BatchProcessingDuration tracks the latency of AutoDream batch processing.
	BatchProcessingDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "ohc_autodream_batch_processing_duration_seconds",
			Help:    "Latency of AutoDream batch processing in seconds.",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"mode", "pipeline"},
	)

	// ConsolidationErrorsTotal tracks the number of errors encountered during consolidation.
	ConsolidationErrorsTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_autodream_consolidation_errors_total",
			Help: "Total number of errors encountered during AutoDream consolidation.",
		},
		[]string{"mode", "pipeline", "error_type"},
	)
)

func init() {
	prometheus.MustRegister(MemoriesProcessedTotal)
	prometheus.MustRegister(BatchProcessingDuration)
	prometheus.MustRegister(ConsolidationErrorsTotal)
}
