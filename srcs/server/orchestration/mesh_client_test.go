package orchestration

import (
	"context"
	"github.com/onehumancorp/mono/srcs/server/orchestration/mesh"
	"testing"
)

type MockMeshClient struct{}

func (m *MockMeshClient) Publish(ctx context.Context, topic string, payload []byte) error {
	return nil
}

func (m *MockMeshClient) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (mesh.Subscription, error) {
	return nil, nil
}

func TestMeshClientInterface(t *testing.T) {
	var client MeshClient = &MockMeshClient{}
	if client == nil {
		t.Fatal("client is nil")
	}
}
