package orchestration

import (
    "context"
    "testing"
    "time"

    "github.com/onehumancorp/mono/src/server/db"
)

func TestMemoryMeshTransport_MeshEvent(t *testing.T) {
    provider := db.NewMockProvider()
    mesh := NewMemoryMeshTransport(provider)

    // Broadcast before subscribe
    err := mesh.BroadcastMeshEvent(context.Background(), "tasks", []byte("test"))
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    ch, err := mesh.SubscribeMeshEvents(context.Background(), "tasks")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = mesh.BroadcastMeshEvent(context.Background(), "tasks", []byte("test2"))
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    select {
    case msg := <-ch:
        if string(msg) != "test2" {
            t.Fatalf("expected test2, got %s", string(msg))
        }
    case <-time.After(1 * time.Second):
        t.Fatalf("timeout waiting for mesh event")
    }
}
