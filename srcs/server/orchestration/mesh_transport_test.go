package orchestration

import (
	"context"
	"testing"
)

type MockHub struct {
	PublishedMessages []Message
}

func (m *MockHub) Publish(msg Message) error {
	m.PublishedMessages = append(m.PublishedMessages, msg)
	return nil
}

func TestMeshTransport_BroadcastStateTransition(t *testing.T) {
	mockHub := &MockHub{}
	transport := NewMeshTransport(mockHub)

	ctx := context.Background()
	err := transport.BroadcastStateTransition(ctx, "evt-1", "ent-1", "PENDING", "COMPLETED")

	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mockHub.PublishedMessages) != 1 {
		t.Fatalf("expected 1 published message, got %d", len(mockHub.PublishedMessages))
	}

	msg := mockHub.PublishedMessages[0]
	if msg.Type != "mesh:coordination" {
		t.Errorf("expected type mesh:coordination, got %s", msg.Type)
	}
}
