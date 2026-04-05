package telemetry

import (
	"go.opentelemetry.io/otel/metric"
)

var (
	AutoDreamSyncDuration  metric.Float64Histogram
	AutoDreamQueryDuration metric.Float64Histogram
	MeshBroadcastTotal     metric.Int64Counter
)

func initKairosMetrics(meter mockableMeter) {
	var err error

	AutoDreamSyncDuration, err = meter.Float64Histogram(
		"ohc_autodream_sync_duration_seconds",
		metric.WithDescription("Duration of AutoDream RAG sync operations"),
	)
	if err != nil {
		// Log error or handle gracefully
	}

	AutoDreamQueryDuration, err = meter.Float64Histogram(
		"ohc_autodream_query_duration_seconds",
		metric.WithDescription("Duration of AutoDream RAG query operations"),
	)
	if err != nil {
		// Log error or handle gracefully
	}

	MeshBroadcastTotal, err = meter.Int64Counter(
		"ohc_mesh_broadcast_total",
		metric.WithDescription("Total number of Teammate Mesh broadcast operations"),
	)
	if err != nil {
		// Log error or handle gracefully
	}
}
