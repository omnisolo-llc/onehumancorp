package perf

import (
	"context"
	"fmt"
	"sync"
	"sync/atomic"
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
	var taskIdx int32 = -1
	errChan := make(chan error, 1)

	for i := 0; i < workerCount; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				idx := atomic.AddInt32(&taskIdx, 1)
				if int(idx) >= len(tasks) {
					return
				}

				select {
				case <-ctx.Done():
					select {
					case errChan <- ctx.Err():
					default:
					}
					return
				default:
					if err := tasks[idx](); err != nil {
						select {
						case errChan <- err:
						default:
						}
					}
				}
			}
		}()
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
	n := 0
	for _, msg := range shard.messages {
		if msg.Recipient == recipient {
			result = append(result, msg)
		} else {
			shard.messages[n] = msg
			n++
		}
	}

	// Clear remaining references to prevent memory leaks
	for i := n; i < len(shard.messages); i++ {
		shard.messages[i] = Message{}
	}

	shard.messages = shard.messages[:n]
	return result
}
