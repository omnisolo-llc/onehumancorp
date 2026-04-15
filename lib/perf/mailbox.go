package perf

import (
	"fmt"
	"sync"
	"time"
)

type Message struct {
	ID        string
	Sender    string
	Recipient string
	Payload   []byte
	Timestamp time.Time
}

type MailboxShard struct {
	mu       sync.Mutex
	messages map[string][]Message
	_        [64]byte // padding to prevent false sharing
}

type ShardedMailbox struct {
	shards []MailboxShard
	mask   uint64
	pool   sync.Pool
}

func NewShardedMailbox(numShards int) *ShardedMailbox {
	size := 1
	for size < numShards {
		size *= 2
	}

	// Contiguous array of shards instead of slice of pointers
	shards := make([]MailboxShard, size)
	for i := 0; i < size; i++ {
		shards[i].messages = make(map[string][]Message)
	}

	return &ShardedMailbox{
		shards: shards,
		mask:   uint64(size - 1),
		pool: sync.Pool{
			New: func() interface{} {
				// Pre-allocate a reasonable capacity for slices
				s := make([]Message, 0, 16)
				return &s
			},
		},
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
	shard := &m.shards[shardIdx]

	shard.mu.Lock()
	msgs := shard.messages[msg.Recipient]
	if msgs == nil {
		// Use pool for new slice allocations
		ptr := m.pool.Get().(*[]Message)
		msgs = (*ptr)[:0] // reset length
	}
	shard.messages[msg.Recipient] = append(msgs, msg)
	shard.mu.Unlock()

	return nil
}

func (m *ShardedMailbox) Read(recipient string) []Message {
	shardIdx := hash(recipient) & m.mask
	shard := &m.shards[shardIdx]

	shard.mu.Lock()
	result := shard.messages[recipient]
	if result != nil {
		// Just clear the map entry, we don't return to pool yet because
		// the caller needs to read the messages.
		// If the caller wants, they can return the slice to the pool later,
		// but in typical usage they drop it and GC picks it up.
		// Alternatively we could allocate a copy, but returning the slice is faster.
		delete(shard.messages, recipient)
	}
	shard.mu.Unlock()

	return result
}

// ReturnSlice allows a caller to return a read slice back to the pool to save GC.
func (m *ShardedMailbox) ReturnSlice(s []Message) {
	if cap(s) > 0 {
		// Optional: clear out references to help GC
		for i := range s {
			s[i] = Message{}
		}
		ptr := &s
		m.pool.Put(ptr)
	}
}
