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
