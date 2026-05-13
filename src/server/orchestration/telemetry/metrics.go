package telemetry

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	SyncDaemonBatchSize = promauto.NewGaugeVec(
		prometheus.GaugeOpts{
			Name: "sync_daemon_batch_size",
			Help: "Number of missions processed in the current batch",
		},
		[]string{"mode"},
	)

	SyncLatency = promauto.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "sync_latency_ms",
			Help:    "Latency of sync operations in milliseconds",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"mode"},
	)

	SyncPayloadSize = promauto.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "sync_payload_size_bytes",
			Help:    "Size of the sync payload in bytes",
			Buckets: prometheus.ExponentialBuckets(100, 2, 10),
		},
		[]string{"mode"},
	)

	SyncDaemonErrorTotal = promauto.NewCounterVec(
		prometheus.CounterOpts{
			Name: "sync_daemon_error_total",
			Help: "Total number of sync errors",
		},
		[]string{"mode", "error"},
	)
)

func RecordSyncDaemonBatchSize(mode string, size int) {
	SyncDaemonBatchSize.WithLabelValues(mode).Set(float64(size))
}

func RecordSyncLatency(mode string, latencyMs float64) {
	SyncLatency.WithLabelValues(mode).Observe(latencyMs)
}

func RecordSyncPayloadSize(mode string, sizeBytes float64) {
	SyncPayloadSize.WithLabelValues(mode).Observe(sizeBytes)
}

func RecordSyncDaemonError(mode string, errorType string) {
	SyncDaemonErrorTotal.WithLabelValues(mode, errorType).Inc()
}
