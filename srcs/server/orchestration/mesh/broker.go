package mesh

import "context"

// MeshBroker defines the contract for broadcasting messages.
type MeshBroker interface {
	Broadcast(ctx context.Context, channel string, payload []byte) error
}
