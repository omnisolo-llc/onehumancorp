package autodream

import (
	"github.com/prometheus/client_golang/prometheus"
)

var (
	// MemoriesProcessedTotal counts the total number of memories successfully processed
	MemoriesProcessedTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "autodream_memories_processed_total",
			Help: "Total number of memories successfully processed",
		},
		[]string{"mode"},
	)

	// BatchProcessingDuration measures the time taken to process a batch of memories
	BatchProcessingDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "autodream_batch_processing_duration_seconds",
			Help:    "Duration of batch processing in seconds",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"mode"},
	)

	// ConsolidationErrorsTotal counts the total number of errors during memory consolidation
	ConsolidationErrorsTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "autodream_consolidation_errors_total",
			Help: "Total number of errors during memory consolidation",
		},
		[]string{"mode"},
	)
)

func init() {
	prometheus.MustRegister(MemoriesProcessedTotal)
	prometheus.MustRegister(BatchProcessingDuration)
	prometheus.MustRegister(ConsolidationErrorsTotal)
}
