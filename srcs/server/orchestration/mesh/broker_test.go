package mesh

import (
	"context"
	"testing"
	"sync"
)

func TestLocalMeshBroker_Broadcast(t *testing.T) {
	localMesh := NewLocalMesh()
	broker := NewLocalMeshBroker(localMesh)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	var wg sync.WaitGroup
	wg.Add(1)

	msgReceived := make(chan string, 1)
	sub, err := localMesh.Subscribe(ctx, "test-channel", func(msg []byte) {
		msgReceived <- string(msg)
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}
	defer sub.Close()

	err = broker.Broadcast(ctx, "test-channel", []byte("test-payload"))
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	wg.Wait()
	received := <-msgReceived
	if received != "test-payload" {
		t.Errorf("Expected 'test-payload', got '%s'", received)
	}
}
