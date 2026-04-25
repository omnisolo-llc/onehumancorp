package nats

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/nats-io/nats.go"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/src/server/integrations/nats")
	meshMessagesPublished metric.Int64Counter
	meshMessagesReceived  metric.Int64Counter
)

func init() {
	var err error
	meshMessagesPublished, err = meter.Int64Counter(
		"ohc.nats.messages_published",
		metric.WithDescription("Number of messages published to the NATS event mesh"),
	)
	if err != nil {
		slog.Error("Failed to initialize NATS published metrics", "error", err)
	}

	meshMessagesReceived, err = meter.Int64Counter(
		"ohc.nats.messages_received",
		metric.WithDescription("Number of messages received from the NATS event mesh"),
	)
	if err != nil {
		slog.Error("Failed to initialize NATS received metrics", "error", err)
	}
}

// NatsMesh implements the interop.TeammateMesh interface using NATS.
type NatsMesh struct {
	nc *nats.Conn
}

// NewNatsMesh creates a new NATS connection and returns a TeammateMesh implementation.
func NewNatsMesh(url string) (*NatsMesh, error) {
	if url == "" {
		url = nats.DefaultURL
	}
	nc, err := nats.Connect(url)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to nats at %s: %w", url, err)
	}
	slog.Info("NATS Event Mesh initialized", "url", url)
	return &NatsMesh{nc: nc}, nil
}

// Publish publishes a message to a NATS subject.
func (m *NatsMesh) Publish(ctx context.Context, subject string, data []byte) error {
	if meshMessagesPublished != nil {
		meshMessagesPublished.Add(ctx, 1, metric.WithAttributes(attribute.String("subject", subject)))
	}
	return m.nc.Publish(subject, data)
}

// Subscribe subscribes to a NATS subject and returns a channel of messages.
func (m *NatsMesh) Subscribe(ctx context.Context, subject string) (<-chan []byte, error) {
	out := make(chan []byte, 100)
	sub, err := m.nc.Subscribe(subject, func(msg *nats.Msg) {
		if meshMessagesReceived != nil {
			meshMessagesReceived.Add(context.Background(), 1, metric.WithAttributes(attribute.String("subject", subject)))
		}
		select {
		case out <- msg.Data:
		case <-ctx.Done():
		}
	})
	if err != nil {
		return nil, fmt.Errorf("failed to subscribe to nats subject %s: %w", subject, err)
	}

	go func() {
		<-ctx.Done()
		err := sub.Unsubscribe()
		if err != nil {
			slog.Error("Error unsubscribing from NATS", "subject", subject, "error", err)
		}
		close(out)
	}()

	return out, nil
}
