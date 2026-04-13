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
	if workerCount <= 0 {
		workerCount = 4
	}
	if len(tasks) < workerCount {
		workerCount = len(tasks)
	}

	// Static chunking gives the absolute best performance for uniform tasks
	chunkSize := (len(tasks) + workerCount - 1) / workerCount

	var wg sync.WaitGroup
	errChan := make(chan error, workerCount)

	ctxDone := ctx.Done()

	for i := 0; i < workerCount; i++ {
		start := i * chunkSize
		end := start + chunkSize
		if start >= len(tasks) {
			break
		}
		if end > len(tasks) {
			end = len(tasks)
		}

		wg.Add(1)
		go func(s, e int) {
			defer wg.Done()
			for j := s; j < e; j++ {
				// Fast context check - only every 16 tasks to minimize select overhead
				if j&15 == 0 {
					select {
					case <-ctxDone:
						select {
						case errChan <- ctx.Err():
						default:
						}
						return
					default:
					}
				}

				if err := tasks[j](); err != nil {
					select {
					case errChan <- err:
					default:
					}
					return
				}
			}
		}(start, end)
	}

	// Wait for completion synchronously to prevent goroutine leaks and race conditions
	wg.Wait()
	close(errChan)

	// Return first error if any
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

// MailboxShard uses padding to avoid false sharing between shards
type MailboxShard struct {
	_        [64]byte // padding
	mu       sync.Mutex
	messages map[string][]Message
	_        [64]byte // padding
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

// fast hash
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
	shard.messages[msg.Recipient] = append(shard.messages[msg.Recipient], msg)
	shard.mu.Unlock()

	return nil
}

func (m *ShardedMailbox) Read(recipient string) []Message {
	shardIdx := hash(recipient) & m.mask
	shard := m.shards[shardIdx]

	shard.mu.Lock()
	result := shard.messages[recipient]
	if len(result) > 0 {
		delete(shard.messages, recipient)
	}
	shard.mu.Unlock()

	return result
}
