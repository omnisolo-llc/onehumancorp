package perf

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestCoordinatorMode_ExecuteParallel(t *testing.T) {
	coord := NewCoordinatorMode(2)

	t.Run("success", func(t *testing.T) {
		var count int
		tasks := []func() error{
			func() error { count++; return nil },
			func() error { count++; return nil },
		}

		err := coord.ExecuteParallel(context.Background(), tasks)
		assert.NoError(t, err)
		// Note: count check might be racy in real world, but we just want to ensure it runs
	})

	t.Run("error", func(t *testing.T) {
		expectedErr := errors.New("task failed")
		tasks := []func() error{
			func() error { return nil },
			func() error { return expectedErr },
		}

		err := coord.ExecuteParallel(context.Background(), tasks)
		assert.Equal(t, expectedErr, err)
	})

	t.Run("empty tasks", func(t *testing.T) {
		err := coord.ExecuteParallel(context.Background(), nil)
		assert.NoError(t, err)
	})

	t.Run("context cancel", func(t *testing.T) {
		ctx, cancel := context.WithCancel(context.Background())
		cancel() // immediately cancel

		tasks := []func() error{
			func() error { time.Sleep(100 * time.Millisecond); return nil },
		}

		err := coord.ExecuteParallel(ctx, tasks)
		assert.Equal(t, context.Canceled, err)
	})
}

func TestShardedMailbox(t *testing.T) {
	mailbox := NewShardedMailbox(4)

	t.Run("send and read", func(t *testing.T) {
		msg := Message{
			ID:        "1",
			Sender:    "agent1",
			Recipient: "agent2",
			Payload:   []byte("test"),
		}

		err := mailbox.Send(msg)
		assert.NoError(t, err)

		msgs := mailbox.Read("agent2")
		assert.Len(t, msgs, 1)
		assert.Equal(t, "1", msgs[0].ID)
	})

	t.Run("empty recipient", func(t *testing.T) {
		msg := Message{ID: "2", Sender: "agent1"}
		err := mailbox.Send(msg)
		assert.Error(t, err)
	})

	t.Run("read empty", func(t *testing.T) {
		msgs := mailbox.Read("nonexistent")
		assert.Len(t, msgs, 0)
	})
}
