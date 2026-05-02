package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"
)

func TestMeshTransports(t *testing.T) {
	memMesh := NewMemoryMeshTransport()

	ch := make(chan MeshEvent, 1)
	err := memMesh.Subscribe(context.Background(), "test-topic", ch)
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}

	event := MeshEvent{
		EventID:   "evt-123",
		Topic:     "test-topic",
		Payload:   []byte("test-payload"),
		Timestamp: time.Now().Unix(),
	}

	err = memMesh.Publish(context.Background(), event)
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	// Wait for event or timeout
	select {
	case received := <-ch:
		if received.EventID != "evt-123" {
			t.Errorf("Expected event evt-123, got %s", received.EventID)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for event")
	}

	// Publish to missing channel
	err = memMesh.Publish(context.Background(), MeshEvent{Topic: "missing-topic"})
	if err != nil {
		t.Fatalf("Failed to publish to missing topic: %v", err)
	}
}

func TestAutoDreamWorker(t *testing.T) {
	worker := NewAutoDreamWorker(&sql.DB{})
	ctx, cancel := context.WithCancel(context.Background())
	worker.Start(ctx)
	worker.processLogs(ctx) // Hit log.Println to ensure coverage of that func
	cancel()
}

func TestTaskDB_Init(t *testing.T) {
	db := NewTaskDB(&sql.DB{}, false)
	if db.isPg {
		t.Error("Expected false for isPg")
	}
}

func TestRedisMeshTransport_NoPanics(t *testing.T) {
    // Basic test ensuring no panic for New since mock requires extra module
    // We cover what we can safely.
    redisMesh := NewRedisMeshTransport(nil)
    ch := make(chan MeshEvent, 1)
    err := redisMesh.Subscribe(context.Background(), "topic", ch)
    if err != nil {
        t.Errorf("Error from dummy subscribe %v", err)
    }
}
