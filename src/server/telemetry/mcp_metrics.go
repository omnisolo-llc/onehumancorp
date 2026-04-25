package telemetry

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	// MCPToolCallsTotal counts the number of tool calls via the MCP.
	MCPToolCallsTotal = promauto.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_mcp_tool_calls_total",
			Help: "Total number of MCP tool calls",
		},
		[]string{"tool_name", "status"},
	)
)
