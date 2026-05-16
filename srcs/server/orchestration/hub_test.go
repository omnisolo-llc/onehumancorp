package orchestration

import (
	"context"
	"testing"
	"time"
)

func TestMemoryMeshTransport(t *testing.T) {
	trans := NewMemoryMeshTransport()
	defer trans.Close()

	received := make(chan MeshMessage, 1)

	cancel, err := trans.Subscribe(context.Background(), "test-topic", func(msg MeshMessage) {
		received <- msg
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	defer cancel()

	msg := MeshMessage{
		AgentID: "agent-1",
		Action:  "test-action",
		Status:  "ok",
		Payload: []byte("test-payload"),
		MsgID:   "msg-1",
	}

	err = trans.Publish(context.Background(), "test-topic", msg)
	if err != nil {
		t.Fatalf("unexpected publish error: %v", err)
	}

	select {
	case got := <-received:
		if got.AgentID != "agent-1" {
			t.Errorf("expected agent-1, got %s", got.AgentID)
		}
	case <-time.After(time.Second):
		t.Fatal("timeout waiting for message")
	}
}

func TestMemoryMeshTransport_EmptyTopic(t *testing.T) {
	trans := NewMemoryMeshTransport()
	defer trans.Close()

	err := trans.Publish(context.Background(), "", MeshMessage{})
	if err != ErrTopicEmpty {
		t.Errorf("expected ErrTopicEmpty, got %v", err)
	}

	_, err = trans.Subscribe(context.Background(), "", func(MeshMessage) {})
	if err != ErrTopicEmpty {
		t.Errorf("expected ErrTopicEmpty, got %v", err)
	}
}
