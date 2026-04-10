package telemetry

import (
	"go.opentelemetry.io/otel/metric"
)

var (
	// MCPOperationsCounter tracks the number of MCP tool executions.
	MCPOperationsCounter metric.Int64Counter

	// MCPOperationDuration tracks the duration of MCP tool executions.
	MCPOperationDuration metric.Float64Histogram
)

func init() {
	// Initialize these lazily when needed or in the main Init() function
	// For now we declare them so they can be injected by Init() if we add them,
	// or they will remain nil and our code handles nil counters.
}
