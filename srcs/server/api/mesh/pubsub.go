package mesh

import "context"

type MeshPubSub interface {
	Publish(ctx context.Context, topic string, message []byte) error
	Subscribe(ctx context.Context, topic string) (<-chan []byte, error)
}
