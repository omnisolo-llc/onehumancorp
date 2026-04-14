package mesh

import "context"

type LocalMeshBroker struct {
	mesh *LocalMesh
}

func NewLocalMeshBroker(mesh *LocalMesh) *LocalMeshBroker {
	return &LocalMeshBroker{mesh: mesh}
}

func (m *LocalMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	return m.mesh.Publish(ctx, channel, payload)
}
