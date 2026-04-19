package agents

import (
	"io"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestInProcessTransport(t *testing.T) {
	r, w := io.Pipe()
	transport := NewInProcessTransport(r, w)

	msg := &Message{
		ID:      "1",
		Content: "hello",
	}

	ch, err := transport.Receive("test-channel")
	assert.NoError(t, err)

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
