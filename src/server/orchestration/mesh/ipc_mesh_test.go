package mesh

import (
	"context"
	"sync"
	"testing"
	"time"
)

func TestIPCMesh_PubSub(t *testing.T) {
	mesh := NewIPCMesh()
	defer mesh.Cleanup()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	var wg sync.WaitGroup
	wg.Add(1)

	msgReceived := make(chan string, 1)

	sub, err := mesh.Subscribe(ctx, "test-topic", func(msg []byte) {
		msgReceived <- string(msg)
		wg.Done()
	})
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	time.Sleep(200 * time.Millisecond)

	err = mesh.Publish(ctx, "test-topic", []byte("hello"))
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
		if <-msgReceived != "hello" {
			t.Errorf("Expected hello, got something else")
		}
	case <-time.After(2 * time.Second):
		t.Fatal("Timeout waiting for message")
	}

	sub.Close()
}
