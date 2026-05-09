package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"github.com/alicebob/miniredis/v2"
	"github.com/redis/rueidis"
)

func TestLocalMeshHub_PublishSubscribe(t *testing.T) {
	hub := NewLocalMeshHub()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	topic := "mesh:events"
	ch, err := hub.Subscribe(ctx, topic)
	require.NoError(t, err)

	msg := []byte("hello local mesh")
	err = hub.Publish(ctx, topic, msg)
	require.NoError(t, err)

	select {
	case received := <-ch:
		assert.Equal(t, msg, received)
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}

func TestRedisMeshHub_PublishSubscribe(t *testing.T) {
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	subClient, err := rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{mr.Addr()}, DisableCache: true})
	require.NoError(t, err)
	defer subClient.Close()

	pubClient, err := rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{mr.Addr()}, DisableCache: true})
	require.NoError(t, err)
	defer pubClient.Close()

	subHub := NewRedisMeshHubWithClient(subClient)
	pubHub := NewRedisMeshHubWithClient(pubClient)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	topic := "mesh:tasks"

	ch, err := subHub.Subscribe(ctx, topic)
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond)

	msg := []byte("hello redis mesh")
	err = pubHub.Publish(ctx, topic, msg)
	require.NoError(t, err)

	select {
	case received := <-ch:
		assert.Equal(t, msg, received)
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}
