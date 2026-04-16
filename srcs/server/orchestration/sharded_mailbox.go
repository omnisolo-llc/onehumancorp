package orchestration

import (
	"sync"
)

type MailboxShard struct {
	mu       sync.Mutex
	messages map[string][]Message
	_        [64]byte // padding to prevent false sharing
}

type ShardedMailbox struct {
	shards []*MailboxShard
	mask   uint64
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

func (m *ShardedMailbox) hash(s string) uint64 {
	const offset64 = 14695981039346656037
	const prime64 = 1099511628211
	h := uint64(offset64)
	for i := 0; i < len(s); i++ {
		h ^= uint64(s[i])
		h *= prime64
	}
	return h
}

func (m *ShardedMailbox) Send(recipient string, msg Message) {
	shardIdx := m.hash(recipient) & m.mask
	shard := m.shards[shardIdx]

	shard.mu.Lock()
	msgs := shard.messages[recipient]
	if cap(msgs) == 0 {
		msgs = getMessageSlice()
	}
	shard.messages[recipient] = append(msgs, msg)
	shard.mu.Unlock()
}

func (m *ShardedMailbox) Read(recipient string) []Message {
	shardIdx := m.hash(recipient) & m.mask
	shard := m.shards[shardIdx]

	shard.mu.Lock()
	result := shard.messages[recipient]
	delete(shard.messages, recipient)
	shard.mu.Unlock()

	return result
}

func (m *ShardedMailbox) Clear(recipient string) {
	shardIdx := m.hash(recipient) & m.mask
	shard := m.shards[shardIdx]

	shard.mu.Lock()
	delete(shard.messages, recipient)
	shard.mu.Unlock()
}
