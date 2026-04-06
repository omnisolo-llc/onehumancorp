package orchestration

import (
	"context"
	"testing"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestMemoryMeshTransport_EventsAndCapabilities(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	pool := db.NewTestProvider(t)
	defer pool.Close()
	mt := NewMemoryMeshTransport(pool)

	t.Run("Capabilities", func(t *testing.T) {
		sub, err := mt.SubscribeCapabilities(ctx)
		require.NoError(t, err)

		err = mt.AdvertiseCapabilities(ctx, pb.AgentCapabilities{
			AgentId: "test-agent",
		})
		require.NoError(t, err)

		select {
		case caps := <-sub:
			assert.Equal(t, "test-agent", caps.AgentId)
		case <-time.After(1 * time.Second):
			t.Fatal("timeout waiting for capabilities")
		}
	})

	t.Run("MeshEvents", func(t *testing.T) {
		sub, err := mt.SubscribeMeshEvents(ctx, "tasks")
		require.NoError(t, err)

		err = mt.BroadcastMeshEvent(ctx, "tasks", []byte("payload"))
		require.NoError(t, err)

		select {
		case payload := <-sub:
			assert.Equal(t, []byte("payload"), payload)
		case <-time.After(1 * time.Second):
			t.Fatal("timeout waiting for mesh event")
		}
	})
}
