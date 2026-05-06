package kairos

import (
	"context"
	"os"
	"sync"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var meter = otel.Meter("onehumancorp/kairos")

var (
	deploymentMode string

	taskQueueLengthGauge      metric.Int64ObservableGauge
	swarmTaskQueueLengthGauge metric.Int64ObservableGauge

	subAgentQueueDelayGauge metric.Float64Histogram
	taskLatencyHistogram    metric.Float64Histogram
	taskThroughputCounter   metric.Float64Counter

	initOnce sync.Once

	taskQueueLengths      sync.Map
	swarmTaskQueueLengths sync.Map
)

func initMetrics() {
	initOnce.Do(func() {
		mode := os.Getenv("OHC_MULTITENANT")
		if mode == "true" {
			deploymentMode = "Cloud"
		} else {
			deploymentMode = "Standalone"
		}

		var err error

		// Asynchronous Gauges for Queue Lengths
		taskQueueLengthGauge, err = meter.Int64ObservableGauge("ohc_task_queue_length")
		if err != nil {
			panic(err)
		}
		_, err = meter.RegisterCallback(func(_ context.Context, o metric.Observer) error {
			taskQueueLengths.Range(func(key, value interface{}) bool {
				mode := key.(string)
				length := value.(int64)
				o.ObserveInt64(taskQueueLengthGauge, length, metric.WithAttributes(
					attribute.String("mode", mode),
					attribute.String("deployment_mode", deploymentMode),
				))
				return true
			})
			return nil
		}, taskQueueLengthGauge)
		if err != nil {
			panic(err)
		}

		swarmTaskQueueLengthGauge, err = meter.Int64ObservableGauge("ohc_swarm_task_queue_length")
		if err != nil {
			panic(err)
		}
		_, err = meter.RegisterCallback(func(_ context.Context, o metric.Observer) error {
			swarmTaskQueueLengths.Range(func(key, value interface{}) bool {
				length := value.(int64)
				o.ObserveInt64(swarmTaskQueueLengthGauge, length, metric.WithAttributes(
					attribute.String("deployment_mode", deploymentMode),
				))
				return true
			})
			return nil
		}, swarmTaskQueueLengthGauge)
		if err != nil {
			panic(err)
		}

		// Synchronous Instruments
		subAgentQueueDelayGauge, err = meter.Float64Histogram("ohc_sub_agent_queue_delay")
		if err != nil {
			panic(err)
		}

		taskLatencyHistogram, err = meter.Float64Histogram("ohc_task_latency")
		if err != nil {
			panic(err)
		}

		taskThroughputCounter, err = meter.Float64Counter("ohc_task_throughput")
		if err != nil {
			panic(err)
		}
	})
}

func getDeploymentMode() string {
	initMetrics()
	return deploymentMode
}

// RecordTaskQueueLength records the current length of the task queue for KAIROS
func RecordTaskQueueLength(ctx context.Context, mode string, length int) error {
	initMetrics()
	taskQueueLengths.Store(mode, int64(length))
	return nil
}

// RecordSwarmTaskQueueLength records the current length of the swarm task queue
func RecordSwarmTaskQueueLength(ctx context.Context, length int) error {
	initMetrics()
	swarmTaskQueueLengths.Store("default", int64(length))
	return nil
}

// RecordSubAgentQueueDelay records the current delay of the sub-agent queue
func RecordSubAgentQueueDelay(ctx context.Context, delay float64) error {
	initMetrics()
	subAgentQueueDelayGauge.Record(ctx, delay, metric.WithAttributes(
		attribute.String("deployment_mode", deploymentMode),
	))
	return nil
}

// RecordTaskLatency records task execution latency differences between Cloud and Standalone modes
func RecordTaskLatency(ctx context.Context, mode string, latencyMs float64) error {
	initMetrics()
	taskLatencyHistogram.Record(ctx, latencyMs, metric.WithAttributes(
		attribute.String("mode", mode),
		attribute.String("deployment_mode", deploymentMode),
	))
	return nil
}

// RecordTaskThroughput records task throughput differences between Cloud and Standalone modes
func RecordTaskThroughput(ctx context.Context, mode string, count float64) error {
	initMetrics()
	taskThroughputCounter.Add(ctx, count, metric.WithAttributes(
		attribute.String("mode", mode),
		attribute.String("deployment_mode", deploymentMode),
	))
	return nil
}
