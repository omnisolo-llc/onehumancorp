package perf

import (
	"context"
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

type CoordinatorMode struct {
	concurrency int
}

func NewCoordinatorMode(concurrency int) *CoordinatorMode {
	if concurrency <= 0 {
		concurrency = 4
	}
	return &CoordinatorMode{
		concurrency: concurrency,
	}
}

func (c *CoordinatorMode) ExecuteParallel(ctx context.Context, tasks []func() error) error {
	if len(tasks) == 0 {
		return nil
	}

	workerCount := c.concurrency
	if len(tasks) < workerCount {
		workerCount = len(tasks)
	}

	var wg sync.WaitGroup
	var idx atomic.Int64
	var firstErr atomic.Value

	for i := 0; i < workerCount; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				if err := ctx.Err(); err != nil {
					firstErr.CompareAndSwap(nil, err)
					return
				}

				curr := idx.Add(1) - 1
				if curr >= int64(len(tasks)) {
					return
				}

				if err := tasks[curr](); err != nil {
					firstErr.CompareAndSwap(nil, err)
					// don't stop worker on error, keep processing tasks, but we record the first error
				}
			}
		}()
	}

	wg.Wait()

	if err := firstErr.Load(); err != nil {
		return err.(error)
	}
	return nil
}

type ShardedMailbox struct {
	shards []*MailboxShard
	mask   uint64
}

type MailboxShard struct {
	mu       sync.Mutex
	messages map[string][]Message
}

type Message struct {
	ID        string
	Sender    string
	Recipient string
	Payload   []byte
	Timestamp time.Time
}

func NewShardedMailbox(numShards int) *ShardedMailbox {
	size := 1
	for size < numShards {
		size *= 2
	}

	shards := make([]*MailboxShard, size)
	for i := 0; i < size; i++ {
		shards[i] = &MailboxShard{
			messages: make(map[string][]Message),
		}
	}

	return &ShardedMailbox{
		shards: shards,
		mask:   uint64(size - 1),
	}
}

func hash(s string) uint64 {
	var h uint64 = 14695981039346656037
	for i := 0; i < len(s); i++ {
		h ^= uint64(s[i])
		h *= 1099511628211
	}
	return h
}

func (m *ShardedMailbox) Send(msg Message) error {
	if msg.Recipient == "" {
		return fmt.Errorf("recipient cannot be empty")
	}

	shardIdx := hash(msg.Recipient) & m.mask
	shard := m.shards[shardIdx]

	shard.mu.Lock()
	msgs := shard.messages[msg.Recipient]
	shard.messages[msg.Recipient] = append(msgs, msg)
	shard.mu.Unlock()

	return nil
}

func (m *ShardedMailbox) Read(recipient string) []Message {
	shardIdx := hash(recipient) & m.mask
	shard := m.shards[shardIdx]

	shard.mu.Lock()
	result := shard.messages[recipient]
	delete(shard.messages, recipient)
	shard.mu.Unlock()

	return result
}
