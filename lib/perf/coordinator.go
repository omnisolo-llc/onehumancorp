package perf

import (
	"context"
	"fmt"
	"sync"
	"time"
)

// CoordinatorMode implements Claude-inspired parallelization for the OHC Team Mesh
// It shards work across multiple goroutines to optimize inter-agent communication latency
type CoordinatorMode struct {
	concurrency int
}

func NewCoordinatorMode(concurrency int) *CoordinatorMode {
	if concurrency <= 0 {
		concurrency = 4 // Default parallelism
	}
	return &CoordinatorMode{
		concurrency: concurrency,
	}
}

// ExecuteParallel runs tasks in parallel sharded chunks
func (c *CoordinatorMode) ExecuteParallel(ctx context.Context, tasks []func() error) error {
	if len(tasks) == 0 {
		return nil
	}

	// Use worker pool approach
	var wg sync.WaitGroup
	taskChan := make(chan func() error, len(tasks))
	errChan := make(chan error, len(tasks))

	// Start workers
	workerCount := c.concurrency
	if len(tasks) < workerCount {
		workerCount = len(tasks)
	}

	for i := 0; i < workerCount; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for task := range taskChan {
				select {
				case <-ctx.Done():
					errChan <- ctx.Err()
					return
				default:
					if err := task(); err != nil {
						errChan <- err
					}
				}
			}
		}()
	}

	// Enqueue tasks
	for _, task := range tasks {
		taskChan <- task
	}
	close(taskChan)

	// Wait for completion
	wg.Wait()
	close(errChan)

	// Collect first error if any
	for err := range errChan {
		if err != nil {
			return err
		}
	}

	return nil
}

// ShardedMailbox represents a highly concurrent mailbox for agent communication
type ShardedMailbox struct {
	shards []*MailboxShard
	mask   uint64
}

type MailboxShard struct {
	mu       sync.RWMutex
	messages []Message
}

type Message struct {
	ID        string
	Sender    string
	Recipient string
	Payload   []byte
	Timestamp time.Time
}

func NewShardedMailbox(numShards int) *ShardedMailbox {
	// Must be power of 2
	size := 1
	for size < numShards {
		size *= 2
	}

	shards := make([]*MailboxShard, size)
	for i := 0; i < size; i++ {
		shards[i] = &MailboxShard{
			messages: make([]Message, 0),
		}
	}

	return &ShardedMailbox{
		shards: shards,
		mask:   uint64(size - 1),
	}
}

// hash string to determine shard
func hash(s string) uint64 {
	var h uint64 = 5381
	for i := 0; i < len(s); i++ {
		h = ((h << 5) + h) + uint64(s[i])
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
	shard.messages = append(shard.messages, msg)
	shard.mu.Unlock()

	return nil
}

func (m *ShardedMailbox) Read(recipient string) []Message {
	shardIdx := hash(recipient) & m.mask
	shard := m.shards[shardIdx]

	shard.mu.Lock()
	defer shard.mu.Unlock()

	// Filter messages for recipient and remove them to prevent memory leaks
	var result []Message
	var remaining []Message
	for _, msg := range shard.messages {
		if msg.Recipient == recipient {
			result = append(result, msg)
		} else {
			remaining = append(remaining, msg)
		}
	}

	shard.messages = remaining
	return result
}
