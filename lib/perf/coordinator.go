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

	workerCount := c.concurrency
	if len(tasks) < workerCount {
		workerCount = len(tasks)
	}

	var wg sync.WaitGroup
	errChan := make(chan error, workerCount)

	chunkSize := (len(tasks) + workerCount - 1) / workerCount

	for i := 0; i < workerCount; i++ {
		start := i * chunkSize
		end := start + chunkSize
		if end > len(tasks) {
			end = len(tasks)
		}
		if start >= len(tasks) {
			break
		}

		wg.Add(1)
		go func(tasksChunk []func() error) {
			defer wg.Done()
			for _, task := range tasksChunk {
				select {
				case <-ctx.Done():
					select {
					case errChan <- ctx.Err():
					default:
					}
					return
				default:
				}
				if err := task(); err != nil {
					select {
					case errChan <- err:
					default:
					}
					return
				}
			}
		}(tasks[start:end])
	}

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
	// Must be power of 2
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
	shard.messages[msg.Recipient] = append(shard.messages[msg.Recipient], msg)
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
