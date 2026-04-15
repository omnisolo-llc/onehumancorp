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
	if bridgeMessagesSentTotal != nil {
		bridgeMessagesSentTotal.Add(ctx, 1)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"metric": "bridge_message_sent",
		}
		RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(payloadMap)
		_ = BufferMetricFunc(ctx, "bridge_message_sent", string(payloadBytes))
	}
}

func RecordBridgeMessageReceived(ctx context.Context) {
	if bridgeMessagesReceivedTotal != nil {
		bridgeMessagesReceivedTotal.Add(ctx, 1)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"metric": "bridge_message_received",
		}
		RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(payloadMap)
		_ = BufferMetricFunc(ctx, "bridge_message_received", string(payloadBytes))
	}
}

func RecordBridgeStatus(ctx context.Context, active int64) {
	if bridgeStatusGauge != nil {
		bridgeStatusGauge.Add(ctx, active)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"metric": "bridge_status",
			"active": active,
		}
		RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(payloadMap)
		_ = BufferMetricFunc(ctx, "bridge_status", string(payloadBytes))
	}
}

func PushMetrics(ctx context.Context, jobName string) error {
	url := os.Getenv("PROMETHEUS_PUSHGATEWAY_URL")
	if url == "" {
		return nil
	}
	return push.New(url, jobName).Gatherer(prometheus.DefaultGatherer).Push()
}
