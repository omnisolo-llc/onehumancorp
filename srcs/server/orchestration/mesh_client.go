package orchestration

import (
	"context"
	"github.com/onehumancorp/mono/srcs/server/orchestration/mesh"
)

type MeshClient interface {
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (mesh.Subscription, error)
}
