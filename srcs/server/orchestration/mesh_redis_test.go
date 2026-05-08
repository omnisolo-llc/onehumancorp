package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestRedisMeshTransport(t *testing.T) {
	mr, err := miniredis.Run()
	require.NoError(t, err)
	defer mr.Close()

	transport, err := NewRedisMeshTransport(mr.Addr())
	require.NoError(t, err)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	channel := "mesh:coordination"
	message := []byte(`{"agent":"test-agent","action":"heartbeat"}`)

	received := make(chan []byte, 1)
	err = transport.Subscribe(ctx, channel, func(data []byte) {
		received <- data
	})
	require.NoError(t, err)

	time.Sleep(200 * time.Millisecond)

	publisher, err := NewRedisMeshTransport(mr.Addr())
	require.NoError(t, err)
	err = publisher.Publish(ctx, channel, message)
	require.NoError(t, err)

	select {
	case data := <-received:
		assert.Equal(t, message, data)
	case <-ctx.Done():
		t.Fatal("timed out waiting for message")
	}
}

func TestNewRedisMeshTransport_Error(t *testing.T) {
	_, err := NewRedisMeshTransport("invalid:address")
	assert.Error(t, err)
}
