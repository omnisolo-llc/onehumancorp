package mesh

import (
	"context"
)

type LocalMeshBroker struct {
	mesh *IPCMesh
}

func NewLocalMeshBroker() *LocalMeshBroker {
	return &LocalMeshBroker{
		mesh: NewIPCMesh(),
	}
}

func (b *LocalMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	return b.mesh.Publish(ctx, channel, payload)
}

func (b *LocalMeshBroker) Subscribe(ctx context.Context, channel string, handler func(msg []byte)) (Subscription, error) {
	return b.mesh.Subscribe(ctx, channel, handler)
}
