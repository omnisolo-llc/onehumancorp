package orchestration

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/centrifugal/centrifuge"
)

func TestCentrifugeNode_MeshHealthCheck_Healthy(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	createNode = func(c centrifuge.Config) (Node, error) {
		return &mockNode{}, nil
	}

	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("unexpected error %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	err = cn.MeshHealthCheck(ctx)
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
}

func TestCentrifugeNode_MeshHealthCheck_PublishError(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	expectedErr := errors.New("broken broker")
	createNode = func(c centrifuge.Config) (Node, error) {
		return &mockNode{errPublish: expectedErr}, nil
	}

	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("unexpected error %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	err = cn.MeshHealthCheck(ctx)
	if err != expectedErr {
		t.Fatalf("expected error %v, got %v", expectedErr, err)
	}
}

func TestCentrifugeNode_MeshHealthCheck_NilNode(t *testing.T) {
	cn := &CentrifugeNode{node: nil}

	ctx, cancel := context.WithTimeout(context.Background(), 1*time.Second)
	defer cancel()

	err := cn.MeshHealthCheck(ctx)
	if !errors.Is(err, context.DeadlineExceeded) {
		t.Fatalf("expected context.DeadlineExceeded error, got %v", err)
	}
}

func TestCentrifugeNode_MeshHealthCheck_ContextDone(t *testing.T) {
	origCreateNode := createNode
	defer func() { createNode = origCreateNode }()

	// create a mock that blocks on publish to simulate timeout
	createNode = func(c centrifuge.Config) (Node, error) {
		return &blockingMockNode{}, nil
	}

	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Fatalf("unexpected error %v", err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // cancel immediately

	err = cn.MeshHealthCheck(ctx)
	if !errors.Is(err, context.Canceled) {
		t.Fatalf("expected context.Canceled error, got %v", err)
	}
}

type blockingMockNode struct {
	mockNode
}

func (m *blockingMockNode) Publish(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
	time.Sleep(1 * time.Second)
	return centrifuge.PublishResult{}, nil
}

func (m *blockingMockNode) Shutdown(ctx context.Context) error {
	return nil
}

func (m *blockingMockNode) Run() error {
	return nil
}

func (m *blockingMockNode) OnConnecting(h centrifuge.ConnectingHandler) {}
func (m *blockingMockNode) OnConnect(h centrifuge.ConnectHandler)       {}
