package local_stateful_execution_proxy

import (
	"context"
	"log"
)

// Emit metrics to OpenTelemetry or any metrics collection system.
// This is a stub implementation meant to demonstrate metrics.
func emitMetrics(ctx context.Context, metricName string, value int) {
	log.Printf("METRIC EMITTED: %s=%d", metricName, value)
}
