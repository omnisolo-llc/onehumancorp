package mesh

import (
	"context"
	"fmt"
		"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestIpcTransport_PublishSubscribe(t *testing.T) {
	dbPath := fmt.Sprintf("file:memdb%d?mode=memory&cache=shared", time.Now().UnixNano())
	transport, err := NewIpcTransport(dbPath)
	assert.NoError(t, err)

	received := make(chan []byte, 1)

	err = transport.Subscribe(context.Background(), "test_channel", func(data []byte) {
		received <- data
	})
	assert.NoError(t, err)

	err = transport.Publish(context.Background(), "test_channel", []byte("hello protobuf"))
	assert.NoError(t, err)

	select {
	case msg := <-received:
		assert.Equal(t, []byte("hello protobuf"), msg)
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for message")
	}
}

func TestIpcTransport_AcquireReleaseLock(t *testing.T) {
	dbPath := fmt.Sprintf("file:memdb%d?mode=memory&cache=shared", time.Now().UnixNano())
	transport, err := NewIpcTransport(dbPath)
	assert.NoError(t, err)

	ctx := context.Background()
	resource := "test_lock"
	owner1 := "owner1"
	owner2 := "owner2"

	ok, err := transport.AcquireLock(ctx, resource, owner1, 10)
	assert.NoError(t, err)
	assert.True(t, ok)

	ok, err = transport.AcquireLock(ctx, resource, owner2, 10)
	assert.NoError(t, err)
	assert.False(t, ok)

	ok, err = transport.AcquireLock(ctx, resource, owner1, 10)
	assert.NoError(t, err)
	assert.True(t, ok) // Renew lock

	err = transport.ReleaseLock(ctx, resource, owner1)
	assert.NoError(t, err)

	ok, err = transport.AcquireLock(ctx, resource, owner2, 10)
	assert.NoError(t, err)
	assert.True(t, ok)
}
