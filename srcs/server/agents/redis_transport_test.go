package agents

import (
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
)

func TestRedisPubSubTransport(t *testing.T) {
	mr, err := miniredis.Run()
	assert.NoError(t, err)
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	transport := NewRedisPubSubTransport(client)

	msg := &Message{
		ID:      "1",
		Content: "hello",
	}

	ch, err := transport.Receive("test-channel")
	assert.NoError(t, err)

	// Wait for subscription to be active
	time.Sleep(100 * time.Millisecond)

	err = transport.Send("test-channel", msg)
	assert.NoError(t, err)

	select {
	case received := <-ch:
		assert.Equal(t, msg.ID, received.ID)
		assert.Equal(t, msg.Content, received.Content)
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}

	err = transport.Close()
	assert.NoError(t, err)
}
