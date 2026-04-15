package mesh

import "context"

type MeshBroker interface {
    Broadcast(ctx context.Context, channel string, payload []byte) error
	Subscribe(ctx context.Context, channel string, handler func(msg []byte)) (Subscription, error)
}
