package mesh

import (
	"context"
	"sync"
	"testing"
)

func TestLocalMeshBroker(t *testing.T) {
	broker := NewLocalMeshBroker()
	ctx := context.Background()

	var wg sync.WaitGroup
	wg.Add(1)

	sub := broker.Subscribe("test-channel")

	go func() {
		msg := <-sub
		if string(msg) != "test-payload" {
			t.Errorf("Expected 'test-payload', got '%s'", string(msg))
		}
		wg.Done()
	}()

	err := broker.Broadcast(ctx, "test-channel", []byte("test-payload"))
	if err != nil {
		t.Fatalf("Broadcast failed: %v", err)
	}

	wg.Wait()
}
