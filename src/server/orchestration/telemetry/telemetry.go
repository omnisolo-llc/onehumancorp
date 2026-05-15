package telemetry


import (
	"context"
	"fmt"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var BufferMetricFunc func(ctx context.Context, name, val string) error

var (
    meshLatency metric.Float64Counter
    meshThroughput metric.Int64Counter
)

func init() {
    meter := otel.Meter("mesh")
    meshLatency, _ = meter.Float64Counter("mesh_latency")
    meshThroughput, _ = meter.Int64Counter("mesh_throughput")
}

func RecordMeshLatency(ctx context.Context, val float64) {
    if BufferMetricFunc != nil {
        BufferMetricFunc(ctx, "mesh_latency", fmt.Sprintf("%v", val))
    } else if meshLatency != nil {
        meshLatency.Add(ctx, val)
    }
}

func RecordMeshThroughput(ctx context.Context, val int) {
    if BufferMetricFunc != nil {
        BufferMetricFunc(ctx, "mesh_throughput", fmt.Sprintf("%v", val))
    } else if meshThroughput != nil {
        meshThroughput.Add(ctx, int64(val))
    }
}
