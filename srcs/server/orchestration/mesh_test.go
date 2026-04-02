package orchestration

import (
	"context"
	"testing"
)

func TestBroadcastMeshMessage(t *testing.T) {
	// A basic test that won't panic when testing the fallback
	hub := NewHub()
	// Without centrifuge node attached, it should just warn and return nil
	err := hub.BroadcastMeshMessage(context.Background(), "room-1", MeshMessage{
		Type:    "TASK_BROADCAST",
		Content: "Hello",
	})
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
}
