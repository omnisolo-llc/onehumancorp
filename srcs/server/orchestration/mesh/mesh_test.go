package mesh

import (
	"context"
	"testing"
	"time"
)

func TestLocalMesh_PubSub(t *testing.T) {
	m := NewLocalMesh()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	received := make(chan []byte, 1)
	sub, err := m.Subscribe(ctx, "test", func(msg []byte) {
		received <- msg
	})
	if err != nil {
		t.Fatal(err)
	}
	defer sub.Close()

	m.Publish(ctx, "test", []byte("hello"))

	select {
	case msg := <-received:
		if string(msg) != "hello" {
			t.Errorf("expected hello, got %s", msg)
		}
	case <-time.After(time.Second):
		t.Error("timeout waiting for message")
	}
}

func TestLocalMesh_Lock(t *testing.T) {
	m := NewLocalMesh()
	ctx := context.Background()

	ok, err := m.AcquireLock(ctx, "key1", 100*time.Millisecond)
	if err != nil || !ok {
		t.Errorf("expected to acquire lock")
	}

	ok, err = m.AcquireLock(ctx, "key1", 100*time.Millisecond)
	if err != nil || ok {
		t.Errorf("expected not to acquire lock")
	}

	m.ReleaseLock(ctx, "key1")

	ok, err = m.AcquireLock(ctx, "key1", 100*time.Millisecond)
	if err != nil || !ok {
		t.Errorf("expected to acquire lock after release")
	}
}

func TestLocalMesh_Presence(t *testing.T) {
	m := NewLocalMesh()
	ctx := context.Background()

	m.RegisterPresence(ctx, "agent1", "IDLE")

	agents, err := m.GetActiveAgents(ctx)
	if err != nil {
		t.Fatal(err)
	}

	if len(agents) != 1 || agents[0].AgentID != "agent1" || agents[0].Status != "IDLE" {
		t.Errorf("unexpected agents: %+v", agents)
	}
}
