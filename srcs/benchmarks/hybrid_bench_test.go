package benchmarks

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/lib/perf"
)

// BenchmarkCoordinatorMode compares parallel vs sequential execution
func BenchmarkCoordinatorMode(b *testing.B) {
	tasks := make([]func() error, 1000)
	for i := 0; i < 1000; i++ {
		tasks[i] = func() error {
			// Simulate work
			time.Sleep(1 * time.Microsecond)
			return nil
		}
	}

	ctx := context.Background()

	b.Run("Sequential", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			for _, task := range tasks {
				_ = task()
			}
		}
	})

	for _, concurrency := range []int{2, 4, 8, 16} {
		b.Run(fmt.Sprintf("Parallel-%d", concurrency), func(b *testing.B) {
			coord := perf.NewCoordinatorMode(concurrency)
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				_ = coord.ExecuteParallel(ctx, tasks)
			}
		})
	}
}

// BenchmarkShardedMailbox compares sharded vs unsharded mailbox
func BenchmarkShardedMailbox(b *testing.B) {
	// Create unsharded (single shard) mailbox for baseline
	unsharded := perf.NewShardedMailbox(1)

	// Create highly sharded mailbox
	sharded := perf.NewShardedMailbox(64)

	b.Run("Unsharded-Write", func(b *testing.B) {
		b.RunParallel(func(pb *testing.PB) {
			i := 0
			for pb.Next() {
				msg := perf.Message{
					ID:        "msg-1",
					Sender:    "agent-1",
					Recipient: fmt.Sprintf("agent-%d", i%100),
					Timestamp: time.Now(),
				}
				_ = unsharded.Send(msg)
				i++
			}
		})
	})

	b.Run("Sharded-Write", func(b *testing.B) {
		b.RunParallel(func(pb *testing.PB) {
			i := 0
			for pb.Next() {
				msg := perf.Message{
					ID:        "msg-1",
					Sender:    "agent-1",
					Recipient: fmt.Sprintf("agent-%d", i%100),
					Timestamp: time.Now(),
				}
				_ = sharded.Send(msg)
				i++
			}
		})
	})
}

// BenchmarkShardedMailboxRead benchmarks the read performance
func BenchmarkShardedMailboxRead(b *testing.B) {
	mailbox := perf.NewShardedMailbox(64)
	// prefill
	for i := 0; i < 10000; i++ {
		msg := perf.Message{
			ID:        "msg-1",
			Sender:    "agent-1",
			Recipient: fmt.Sprintf("agent-%d", i%1000),
			Timestamp: time.Now(),
		}
		_ = mailbox.Send(msg)
	}

	b.ResetTimer()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			_ = mailbox.Read(fmt.Sprintf("agent-%d", i%1000))
			// re-insert to keep mailbox populated
			msg := perf.Message{
				ID:        "msg-1",
				Sender:    "agent-1",
				Recipient: fmt.Sprintf("agent-%d", i%1000),
				Timestamp: time.Now(),
			}
			_ = mailbox.Send(msg)
			i++
		}
	})
}
