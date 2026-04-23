package telemetry

import (
	"context"

	"go.opentelemetry.io/otel/metric"
)

var (
	// WebhookRelayReceivedTotal counts the number of webhooks received by the relay
	WebhookRelayReceivedTotal metric.Int64Counter

	// WebhookRelayForwardedTotal counts the number of webhooks successfully forwarded to local agents
	WebhookRelayForwardedTotal metric.Int64Counter
)

func initWebhookMetrics(m mockableMeter) error {
	var err error
	WebhookRelayReceivedTotal, err = m.Int64Counter(
		"ohc_webhook_relay_received_total",
		metric.WithDescription("Total number of webhooks received by the cloud relay"),
	)
	if err != nil {
		return err
	}

	WebhookRelayForwardedTotal, err = m.Int64Counter(
		"ohc_webhook_relay_forwarded_total",
		metric.WithDescription("Total number of webhooks forwarded to local agents"),
	)
	if err != nil {
		return err
	}

	return nil
}

func RecordWebhookRelayReceived(ctx context.Context) {
	if BufferMetricFunc != nil {
		_ = BufferMetricFunc(ctx, "webhook_relay_received", "{}")
	}
	if WebhookRelayReceivedTotal != nil {
		WebhookRelayReceivedTotal.Add(ctx, 1)
	}
}

func RecordWebhookRelayForwarded(ctx context.Context) {
	if BufferMetricFunc != nil {
		_ = BufferMetricFunc(ctx, "webhook_relay_forwarded", "{}")
	}
	if WebhookRelayForwardedTotal != nil {
		WebhookRelayForwardedTotal.Add(ctx, 1)
	}
}
