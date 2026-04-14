package mesh

import (
	"context"
)

// MeshBroker provides a unified interface for broadcasting real-time events over the Teammate Mesh.
type MeshBroker interface {
	// Broadcast publishes a JSON payload to the specified channel.
	Broadcast(ctx context.Context, channel string, payload []byte) error
}
