package telemetry

import (
	"context"
	"encoding/json"
	"os"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/push"
	"go.opentelemetry.io/otel/metric"
)

var (
	bridgeMessagesSentTotal     metric.Int64Counter
	bridgeMessagesReceivedTotal metric.Int64Counter
	bridgeStatusGauge           metric.Int64UpDownCounter
)

func initBridgeMetrics() {
	var err error
	if meter == nil {
		return // telemetry not fully initialized, wait for InitTelemetry
	}

	bridgeMessagesSentTotal, err = meter.Int64Counter(
		"ohc_mesh_bridge_messages_sent_total",
		metric.WithDescription("Total number of Teammate Mesh bridge messages sent"),
	)
	if err != nil {
		panic(err)
	}

	bridgeMessagesReceivedTotal, err = meter.Int64Counter(
		"ohc_mesh_bridge_messages_received_total",
		metric.WithDescription("Total number of Teammate Mesh bridge messages received"),
	)
	if err != nil {
		panic(err)
	}

	bridgeStatusGauge, err = meter.Int64UpDownCounter(
		"ohc_mesh_bridge_status_gauge",
		metric.WithDescription("Active Teammate Mesh bridge connections"),
	)
	if err != nil {
		panic(err)
	}
}

func RecordBridgeMessageSent(ctx context.Context) {
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{}
		redactedMap := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redactedMap)
		_ = BufferMetricFunc(ctx, "ohc_mesh_bridge_messages_sent_total", string(payloadBytes))
	}
	if bridgeMessagesSentTotal != nil {
		bridgeMessagesSentTotal.Add(ctx, 1)
	}
}

func RecordBridgeMessageReceived(ctx context.Context) {
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{}
		redactedMap := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redactedMap)
		_ = BufferMetricFunc(ctx, "ohc_mesh_bridge_messages_received_total", string(payloadBytes))
	}
	if bridgeMessagesReceivedTotal != nil {
		bridgeMessagesReceivedTotal.Add(ctx, 1)
	}
}

func RecordBridgeStatus(ctx context.Context, active int64) {
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"active": active,
		}
		redactedMap := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redactedMap)
		_ = BufferMetricFunc(ctx, "ohc_mesh_bridge_status_gauge", string(payloadBytes))
	}
	if bridgeStatusGauge != nil {
		bridgeStatusGauge.Add(ctx, active)
	}
}

func PushMetrics(ctx context.Context, jobName string) error {
	url := os.Getenv("PROMETHEUS_PUSHGATEWAY_URL")
	if url == "" {
		return nil
	}
	return push.New(url, jobName).Gatherer(prometheus.DefaultGatherer).Push()
}
