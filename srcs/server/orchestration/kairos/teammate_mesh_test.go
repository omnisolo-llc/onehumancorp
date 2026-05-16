package kairos

import (
	"context"
	"testing"
	"time"
)

func TestLocalTeammateMesh(t *testing.T) {
	mesh := NewLocalTeammateMesh()
	ctx := context.Background()

	t.Run("Publish and Subscribe", func(t *testing.T) {
		sub, err := mesh.Subscribe(ctx, "test_topic", func(msg []byte) {})
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		err = mesh.Publish(ctx, "test_topic", []byte("hello"))
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		sub.Unsubscribe()
	})

	t.Run("Acquire and Release Lock", func(t *testing.T) {
		ok, err := mesh.AcquireLock(ctx, "test_lock", time.Second)
		if err != nil || !ok {
			t.Fatalf("expected to acquire lock")
		}

		err = mesh.ReleaseLock(ctx, "test_lock")
		if err != nil {
			t.Fatalf("expected to release lock")
		}
	})

	t.Run("Register Presence and Get Active Agents", func(t *testing.T) {
		err := mesh.RegisterPresence(ctx, "agent_1", "online")
		if err != nil {
			t.Fatalf("expected no error")
		}

		agents, err := mesh.GetActiveAgents(ctx)
		if err != nil || len(agents) != 1 || agents[0].AgentID != "agent_1" {
			t.Fatalf("expected to get active agents")
		}
	})

	t.Run("Acknowledge", func(t *testing.T) {
		err := mesh.Acknowledge(ctx, "msg_1")
		if err != nil {
			t.Fatalf("expected no error")
		}
	})
}
