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

	// Dynamically calculate batch size to ensure all workers get tasks
	batchSize := int64(len(tasks) / (workerCount * 4))
	if batchSize < 1 {
		batchSize = 1
	}
	// Cap maximum batch size to prevent starvation on very large queues
	if batchSize > 64 {
		batchSize = 64
	}

	for i := 0; i < workerCount; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				if err := ctx.Err(); err != nil {
					firstErr.CompareAndSwap(nil, err)
					return
				}

				endIdx := idx.Add(batchSize)
				startIdx := endIdx - batchSize

				if startIdx >= int64(len(tasks)) {
					return
				}

				if endIdx > int64(len(tasks)) {
					endIdx = int64(len(tasks))
				}

				for curr := startIdx; curr < endIdx; curr++ {
					if err := tasks[curr](); err != nil {
						firstErr.CompareAndSwap(nil, err)
					}
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
	_        [64]byte // padding to prevent false sharing
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

// hash uses FNV-1a hash algorithm
func hash(s string) uint64 {
	const offset64 = 14695981039346656037
	const prime64 = 1099511628211
	h := uint64(offset64)
	for i := 0; i < len(s); i++ {
		h ^= uint64(s[i])
		h *= prime64
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
