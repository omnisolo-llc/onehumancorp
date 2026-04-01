package telemetry

// BufferMetricFunc is a dependency-injected function pointer that handles buffering
// telemetry metrics locally when operating in Standalone Mode.
// It receives the metric type and a JSON-marshaled payload string.
var BufferMetricFunc func(metricType string, payload string)
