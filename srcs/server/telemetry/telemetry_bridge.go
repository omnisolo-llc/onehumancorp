package telemetry

import (
	"context"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/push"
	"go.opentelemetry.io/otel/metric"
	"net/http"
	"os"
	"time"
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
}

func RecordBridgeMessageReceived(ctx context.Context) {
	if bridgeMessagesReceivedTotal != nil {
		bridgeMessagesReceivedTotal.Add(ctx, 1)
	}
}

func RecordBridgeStatus(ctx context.Context, active int64) {
	if bridgeStatusGauge != nil {
		bridgeStatusGauge.Add(ctx, active)
	}
}

func PushMetrics(ctx context.Context, jobName string) error {
	url := os.Getenv("PROMETHEUS_PUSHGATEWAY_URL")
	if url == "" {
		url = "http://localhost:9091"
	}
	// Since OpenTelemetry manages its own prometheus exporter,
	// we directly use the prometheus DefaultGatherer to ensure all exported OTEL metrics
	// are successfully pushed to the pushgateway.
	pusher := push.New(url, jobName).Gatherer(prometheus.DefaultGatherer)

	// Create an HTTP client with a strict timeout to prevent leaking goroutines
	// if the Pushgateway is slow or unresponsive.
	client := &http.Client{
		Timeout: 5 * time.Second,
	}
	pusher.Client(client)

	// Execute push asynchronously to avoid blocking orchestration threads
	go func() {
		_ = pusher.Push()
	}()

	return nil
}
