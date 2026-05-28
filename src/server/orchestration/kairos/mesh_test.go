package kairos

import (
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
)

func TestMemoryMesh_PublishSubscribe(t *testing.T) {
	mesh := NewMemoryMesh()
	channel := "mesh:tasks"

	// Publish with no subscribers
	if err := mesh.Publish(channel, []byte("ignored")); err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	subCh, err := mesh.Subscribe(channel)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	msg := []byte("hello world")
	if err := mesh.Publish(channel, msg); err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	select {
	case received := <-subCh:
		if string(received) != string(msg) {
			t.Errorf("Expected %s, got %s", msg, received)
		}
	case <-time.After(1 * time.Second):
		t.Error("Timeout waiting for message")
	}
}

func TestRedisMesh_PublishSubscribe(t *testing.T) {
	// Start miniredis
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer s.Close()

	// Use redis url pointing to miniredis
	redisURL := "redis://" + s.Addr()

	mesh, err := NewRedisMesh(redisURL)
	if err != nil {
		t.Fatalf("Failed to create Redis mesh: %v", err)
	}

	// Ensure cleanup
	defer mesh.client.Close()

	channel := "mesh:coordination"

	subCh, err := mesh.Subscribe(channel)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	msg := []byte("hello redis")
	if err := mesh.Publish(channel, msg); err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	select {
	case received := <-subCh:
		if string(received) != string(msg) {
			t.Errorf("Expected %s, got %s", msg, received)
		}
	case <-time.After(2 * time.Second):
		t.Error("Timeout waiting for message")
	}
}

func TestRedisMesh_ConnectionError(t *testing.T) {
	_, err := NewRedisMesh("invalid-url")
	if err == nil {
		t.Error("Expected error for invalid redis url")
	}

	_, err = NewRedisMesh("redis://localhost:1")
	if err == nil {
		t.Error("Expected error for connection refused")
	}
}

func TestRedisMesh_SubscribeError(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	redisURL := "redis://" + s.Addr()

	mesh, err := NewRedisMesh(redisURL)
	if err != nil {
		t.Fatalf("Failed to create Redis mesh: %v", err)
	}
	mesh.client.Close() // Force a close to cause Subscribe to fail
	s.Close()

	_, err = mesh.Subscribe("some-channel")
	if err == nil {
		t.Error("Expected error for Subscribe after close")
	}
}
