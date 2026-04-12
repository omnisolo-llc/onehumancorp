package mesh

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestLocalMesh(t *testing.T) {
	mesh := NewLocalMesh()
	testMesh(t, mesh)
}

// Skipping RedisMesh test without miniredis or a real redis instance.
// In a real scenario, an integration test would connect to a real Redis container.

func testMesh(t *testing.T, mesh TeammateMesh) {
	ctx := context.Background()

	t.Run("PubSub", func(t *testing.T) {
		topic := "test-topic"
		msg := []byte("hello")
		received := make(chan []byte, 1)

		sub, err := mesh.Subscribe(ctx, topic, func(m []byte) {
			received <- m
		})
		require.NoError(t, err)
		defer sub.Unsubscribe(ctx)

		// Yield to allow subscription to register
		time.Sleep(10 * time.Millisecond)

		err = mesh.Publish(ctx, topic, msg)
		require.NoError(t, err)

		select {
		case m := <-received:
			assert.Equal(t, msg, m)
		case <-time.After(time.Second):
			t.Fatal("Timeout waiting for message")
		}
	})

	t.Run("Locking", func(t *testing.T) {
		key := "test-lock"

		// First acquire should succeed
		acquired, err := mesh.AcquireLock(ctx, key, time.Minute)
		require.NoError(t, err)
		assert.True(t, acquired)

		// Second acquire should fail
		acquired, err = mesh.AcquireLock(ctx, key, time.Minute)
		require.NoError(t, err)
		assert.False(t, acquired)

		// Release lock
		err = mesh.ReleaseLock(ctx, key)
		require.NoError(t, err)

		// Third acquire should succeed again
		acquired, err = mesh.AcquireLock(ctx, key, time.Minute)
		require.NoError(t, err)
		assert.True(t, acquired)
	})

	t.Run("Presence", func(t *testing.T) {
		agentID := "agent-1"
		status := "IDLE"

		err := mesh.RegisterPresence(ctx, agentID, status)
		require.NoError(t, err)

		agents, err := mesh.GetActiveAgents(ctx)
		require.NoError(t, err)
		assert.Len(t, agents, 1)
		assert.Equal(t, agentID, agents[0].AgentID)
		assert.Equal(t, status, agents[0].Status)
	})
}
