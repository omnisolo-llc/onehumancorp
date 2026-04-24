package interop

import (
	"context"
	"sync"
	"testing"
	"time"
)

func TestMemoryMesh_PubSub(t *testing.T) {
	mesh := NewTeammateMeshWithClient(nil)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	channel := "test_channel"

	sub, err := mesh.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	msg := []byte("hello swarm")
	if err := mesh.Publish(ctx, channel, msg); err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	select {
	case rcv := <-sub:
		if string(rcv) != string(msg) {
			t.Errorf("expected %s, got %s", string(msg), string(rcv))
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timed out waiting for message")
	}
}

func TestMemoryMesh_MultipleSubscribers(t *testing.T) {
	mesh := NewTeammateMeshWithClient(nil)
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	channel := "multi_channel"

	sub1, err := mesh.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("failed to subscribe sub1: %v", err)
	}

	sub2, err := mesh.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("failed to subscribe sub2: %v", err)
	}

	msg := []byte("broadcast message")
	if err := mesh.Publish(ctx, channel, msg); err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		defer wg.Done()
		select {
		case rcv := <-sub1:
			if string(rcv) != string(msg) {
				t.Errorf("sub1 expected %s, got %s", string(msg), string(rcv))
			}
		case <-time.After(1 * time.Second):
			t.Error("sub1 timed out waiting for message")
		}
	}()

	go func() {
		defer wg.Done()
		select {
		case rcv := <-sub2:
			if string(rcv) != string(msg) {
				t.Errorf("sub2 expected %s, got %s", string(msg), string(rcv))
			}
		case <-time.After(1 * time.Second):
			t.Error("sub2 timed out waiting for message")
		}
	}()

	wg.Wait()
}
