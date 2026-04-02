package telemetry
import (
    "context"
    "go.opentelemetry.io/otel/metric"
)

var swarmTasksCompleted metric.Int64Counter

func init() {
    if meter != nil {
        swarmTasksCompleted, _ = meter.Int64Counter("ohc_swarm_tasks_completed", metric.WithDescription("Number of swarm tasks completed"))
    }
}

func RecordSwarmTaskCompleted(ctx context.Context) {
    if swarmTasksCompleted != nil {
        swarmTasksCompleted.Add(ctx, 1)
    }
}
