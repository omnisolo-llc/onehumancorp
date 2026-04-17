package kairos

import (
	"os"

	"github.com/prometheus/client_golang/prometheus"
)

var (
	// TransitionsTotal tracks the total number of KAIROS state transitions.
	TransitionsTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_kairos_transitions_total",
			Help: "Total number of KAIROS state machine transitions.",
		},
		[]string{"mode", "status"},
	)

	// TransitionDuration tracks the latency of KAIROS state transitions.
	TransitionDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "ohc_kairos_transition_duration_seconds",
			Help:    "Latency of KAIROS state transitions in seconds.",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"mode"},
	)

	// TaskQueueDepth tracks the current depth of the Sub-Agent task queue.
	TaskQueueDepth = prometheus.NewGaugeVec(
		prometheus.GaugeOpts{
			Name: "ohc_agent_task_queue_depth",
			Help: "Current depth of the KAIROS Sub-Agent task queue.",
		},
		[]string{"mode"},
	)

	AutoDreamEmbeddingDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "ohc_autodream_embedding_duration_seconds",
			Help:    "Latency of the embedding LLM endpoint.",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"mode"},
	)

	AutoDreamStorageOpsTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_autodream_storage_operations_total",
			Help: "Vector DB insert success/failures, tagged by db_type (pgvector vs sqlite) and status.",
		},
		[]string{"mode", "db_type", "status"},
	)

	AutoDreamWorkerTasksTotal = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_autodream_worker_tasks_total",
			Help: "Tracking tasks processed by the AutoDream worker.",
		},
		[]string{"mode", "status"},
	)
)

func init() {
	prometheus.MustRegister(TransitionsTotal)
	prometheus.MustRegister(TransitionDuration)
	prometheus.MustRegister(TaskQueueDepth)
	prometheus.MustRegister(AutoDreamEmbeddingDuration)
	prometheus.MustRegister(AutoDreamStorageOpsTotal)
	prometheus.MustRegister(AutoDreamWorkerTasksTotal)
}

// GetMode returns the current execution mode of the OHC Hybrid OS.
func GetMode() string {
	if os.Getenv("OHC_HEADLESS") == "true" {
		return "headless"
	}
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return "cloud"
	}
	return "standalone"
}
