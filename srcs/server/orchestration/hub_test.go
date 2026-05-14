package orchestration

import (
	"context"
	"testing"
	"time"
)

func TestMemoryMeshTransport(t *testing.T) {
	transport := NewMemoryMeshTransport()

	ch := make(chan []byte, 1)
	handler := func(payload []byte) {
		ch <- payload
	}

	transport.Subscribe(context.Background(), "test.topic", handler)

	err := transport.Publish(context.Background(), "test.topic", []byte("hello"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	select {
	case payload := <-ch:
		if string(payload) != "hello" {
			t.Fatalf("expected hello, got %s", string(payload))
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}
}

func TestTaskManager(t *testing.T) {
	transport := NewMemoryMeshTransport()
	node := NewCentrifugeNode(transport)
	tm := NewTaskManager(node)

	ch := make(chan []byte, 1)
	handler := func(payload []byte) {
		ch <- payload
	}

	transport.Subscribe(context.Background(), "task.created", handler)

	err := tm.CreateTask(context.Background(), "123", []byte("payload data"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	select {
	case payload := <-ch:
		if string(payload) != "payload data" {
			t.Fatalf("expected payload data, got %s", string(payload))
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}
}

func TestRedisMeshTransport_InitError(t *testing.T) {
	_, err := NewRedisMeshTransport("invalid://url")
	if err == nil {
		t.Fatal("expected error with invalid url")
	}
}
