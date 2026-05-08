package orchestration

import "context"

// MeshTransport defines the interface for the highly available realtime communication layer.
type MeshTransport interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string, handler func(data []byte)) error
}
