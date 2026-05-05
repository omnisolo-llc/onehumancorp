package telemetry

import (
	"context"
	"net/http"
	"time"

	"onehumancorp/srcs/server/db"
)

// StartStandaloneDaemons initializes and starts the standalone telemetry daemons
func StartStandaloneDaemons(ctx context.Context, provider db.Provider, endpointURL string) {
	// Initialize the exporter to collect telemetry locally
	exporter := NewSQLiteExporter(provider)

	// In a complete integration, we would register this exporter with the OpenTelemetry SDK
	// For example:
	// metricProvider := metric.NewMeterProvider(metric.WithReader(metric.NewPeriodicReader(exporter)))
	// global.SetMeterProvider(metricProvider)

	// Just logging to ensure it doesn't complain about unused variable
	_ = exporter

	// Start the sync worker to push telemetry to the cloud
	worker := NewMcpSyncWorker(provider, 60*time.Second, endpointURL, &http.Client{Timeout: 10 * time.Second})
	go worker.Start(ctx)
}
