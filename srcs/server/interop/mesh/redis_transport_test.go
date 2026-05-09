package mesh

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/stretchr/testify/assert"
)

func TestRedisTransport_PublishSubscribe(t *testing.T) {
	s, err := miniredis.Run()
	assert.NoError(t, err)
	defer s.Close()

	transport, err := NewRedisTransport("redis://" + s.Addr())
	assert.NoError(t, err)

	received := make(chan []byte, 1)

	err = transport.Subscribe(context.Background(), "test_redis", func(data []byte) {
		received <- data
	})
	assert.NoError(t, err)

	// small delay for subscription to establish
	time.Sleep(50 * time.Millisecond)

	err = transport.Publish(context.Background(), "test_redis", []byte("hello redis"))
	assert.NoError(t, err)

	select {
	case msg := <-received:
		assert.Equal(t, []byte("hello redis"), msg)
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for redis message")
	}
}

func TestRedisTransport_AcquireReleaseLock(t *testing.T) {
	s, err := miniredis.Run()
	assert.NoError(t, err)
	defer s.Close()

	transport, err := NewRedisTransport("redis://" + s.Addr())
	assert.NoError(t, err)

	ctx := context.Background()
	resource := "test_redis_lock"
	owner1 := "owner1"
	owner2 := "owner2"

	ok, err := transport.AcquireLock(ctx, resource, owner1, 10)
	assert.NoError(t, err)
	assert.True(t, ok)

	ok, err = transport.AcquireLock(ctx, resource, owner2, 10)
	assert.NoError(t, err)
	assert.False(t, ok)

	err = transport.ReleaseLock(ctx, resource, owner1)
	assert.NoError(t, err)

	ok, err = transport.AcquireLock(ctx, resource, owner2, 10)
	assert.NoError(t, err)
	assert.True(t, ok)
}
