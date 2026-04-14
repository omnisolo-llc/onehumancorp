package orchestration

import (
	"context"
	"fmt"
	"log/slog"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	transportMeter = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration")
	meshBroadcasts, _ = transportMeter.Int64Counter("mesh.transport.broadcasts")
)

type CentrifugeHub interface {
	Publish(msg Message) error
}

type MeshTransportImpl struct {
	hub CentrifugeHub
}

func NewMeshTransport(hub CentrifugeHub) *MeshTransportImpl {
	return &MeshTransportImpl{
		hub: hub,
	}
}

func (m *MeshTransportImpl) BroadcastStateTransition(ctx context.Context, eventID string, entityID string, fromState string, toState string) error {
	meshBroadcasts.Add(ctx, 1)

	payload := fmt.Sprintf(`{"event_id":"%s","entity_id":"%s","from_state":"%s","to_state":"%s"}`, eventID, entityID, fromState, toState)

	msg := Message{
		ID:        eventID,
		FromAgent: "system",
		ToAgent:   "system",
		Type:      "mesh:coordination",
		Content:   payload,
	}

	if m.hub != nil {
		err := m.hub.Publish(msg)
		if err != nil {
			slog.ErrorContext(ctx, "Failed to broadcast state transition", "err", err, "eventID", eventID)
			return err
		}
	}
	return nil
}
