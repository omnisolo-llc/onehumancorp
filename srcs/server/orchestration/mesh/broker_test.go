package mesh

import (
	"context"
	"testing"
)

func TestLocalBroker(t *testing.T) {
	broker := NewLocalMeshBroker()
	err := broker.Broadcast(context.Background(), "test-channel", []byte("test"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}
