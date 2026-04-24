package interop

import (
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/interop")

	meshMessagesPublished, _ = meter.Int64Counter(
		"mesh_messages_published",
		metric.WithDescription("Number of messages published to the Teammate Mesh"),
	)
	meshMessagesReceived, _ = meter.Int64Counter(
		"mesh_messages_received",
		metric.WithDescription("Number of messages received from the Teammate Mesh"),
	)
)
