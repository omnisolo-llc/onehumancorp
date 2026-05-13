package orchestration

import (
	"context"
	"sync"
	"testing"
	"time"

	pb "github.com/onehumancorp/ohc/srcs/proto"
)

func TestMemoryMeshTransport_PublishSubscribe(t *testing.T) {
	transport := NewMemoryMeshTransport()
	defer transport.Close()

	ctx := context.Background()
	channel := "test-channel"

	var wg sync.WaitGroup
	wg.Add(2)

	var received1, received2 *pb.MeshEvent
	var mu sync.Mutex

	err := transport.Subscribe(ctx, channel, func(event *pb.MeshEvent) {
		mu.Lock()
		received1 = event
		mu.Unlock()
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Failed to subscribe 1: %v", err)
	}

	err = transport.Subscribe(ctx, channel, func(event *pb.MeshEvent) {
		mu.Lock()
		received2 = event
		mu.Unlock()
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Failed to subscribe 2: %v", err)
	}

	event := &pb.MeshEvent{
		Id:   "event-1",
		Type: "test",
	}

	err = transport.Publish(ctx, channel, event)
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	// Wait with timeout
	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
	case <-time.After(time.Second):
		t.Fatal("Timeout waiting for events")
	}

	mu.Lock()
	defer mu.Unlock()

	if received1 == nil || received1.Id != event.Id {
		t.Errorf("Receiver 1 got unexpected event: %v", received1)
	}
	if received2 == nil || received2.Id != event.Id {
		t.Errorf("Receiver 2 got unexpected event: %v", received2)
	}
}

func TestMemoryMeshTransport_PublishNoSubscribers(t *testing.T) {
	transport := NewMemoryMeshTransport()
	defer transport.Close()

	ctx := context.Background()
	err := transport.Publish(ctx, "empty-channel", &pb.MeshEvent{Id: "1"})
	if err != nil {
		t.Fatalf("Expected no error publishing to empty channel, got: %v", err)
	}
}

func TestMemoryMeshTransport_Close(t *testing.T) {
	transport := NewMemoryMeshTransport()

	err := transport.Close()
	if err != nil {
		t.Fatalf("Failed to close: %v", err)
	}

	err = transport.Subscribe(context.Background(), "chan", func(e *pb.MeshEvent) {})
	if err == nil {
		t.Fatal("Expected error subscribing to closed transport")
	}
}
