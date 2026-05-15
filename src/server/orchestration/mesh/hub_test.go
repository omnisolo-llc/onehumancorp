package mesh

import (
	"context"

	"testing"
	"time"
    "sync"

	hub "github.com/onehumancorp/mono/src/proto/hub"
)

func TestMemoryMeshTransport(t *testing.T) {
	transport := NewMemoryMeshTransport()
	ctx := context.Background()

    var wg sync.WaitGroup
    wg.Add(1)

    transport.Subscribe(ctx, "test", func(e *hub.MeshEvent) {
        if e.EventId != "1" {
            t.Errorf("expected 1")
        }
        wg.Done()
    })

	event := &hub.MeshEvent{
		EventId:   "1",
		Topic:     "test",
		Payload:   []byte("data"),
		Timestamp: time.Now().Unix(),
	}

	err := transport.Publish(ctx, "test", event)
	if err != nil {
		t.Errorf("Publish failed: %v", err)
	}

    wg.Wait()

	locked, err := transport.AcquireLock(ctx, "res", "owner", time.Second)
	if err != nil || !locked {
		t.Errorf("AcquireLock failed")
	}

	err = transport.ReleaseLock(ctx, "res", "owner")
	if err != nil {
		t.Errorf("ReleaseLock failed")
	}
}

func TestCentrifugeNode(t *testing.T) {
	transport := NewMemoryMeshTransport()
	node := NewCentrifugeNode(transport)
	ctx := context.Background()



	err := node.Broadcast(ctx, "test", &hub.MeshEvent{})
	if err != nil {
		t.Errorf("Broadcast failed: %v", err)
	}
}

func TestRedisMeshTransport(t *testing.T) {
	transport, err := NewRedisMeshTransport([]string{"127.0.0.1:0"})
	if err == nil && transport != nil {
        ctx := context.Background()
		_ = transport.Publish(ctx, "test", &hub.MeshEvent{})
		_ = transport.Subscribe(ctx, "test", func(e *hub.MeshEvent) {})
		_, _ = transport.AcquireLock(ctx, "res", "owner", time.Second)
		_ = transport.ReleaseLock(ctx, "res", "owner")
	}
}
