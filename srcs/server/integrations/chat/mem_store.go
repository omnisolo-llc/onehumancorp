package chat

import (
	"context"
	"fmt"
	"sort"
	"sync"
	"sync/atomic"
	"time"
)

// MemStore is a thread-safe in-memory implementation of Store.
// Designed for tests and environments without a Redis connection.
// Suitable for single-process replay (e.g. debugging sessions).
type MemStore struct {
	mu       sync.RWMutex
	messages []Message
	counter  atomic.Int64
}

// NewMemStore returns a new in-memory chat store.
func NewMemStore() *MemStore {
	return &MemStore{}
}

// Append appends a message and returns a synthetic stream-style ID.
func (s *MemStore) Append(_ context.Context, msg Message) (string, error) {
	if msg.ConversationID == "" {
		return "", fmt.Errorf("chat: conversation_id is required")
	}
	if msg.Timestamp.IsZero() {
		msg.Timestamp = time.Now().UTC()
	}
	seq := s.counter.Add(1)
	msg.ID = fmt.Sprintf("%d-%d", msg.Timestamp.UnixMilli(), seq)

	s.mu.Lock()
	s.messages = append(s.messages, msg)
	s.mu.Unlock()

	return msg.ID, nil
}

// Replay returns all messages for the given conversationID in insertion order.
// When conversationID is empty all messages are returned.
// limit <= 0 means no limit.
func (s *MemStore) Replay(_ context.Context, conversationID string, limit int64) ([]Message, error) {
	s.mu.RLock()
	all := make([]Message, len(s.messages))
	copy(all, s.messages)
	s.mu.RUnlock()

	// Sort by timestamp (already insertion order, but sort for stability).
	sort.Slice(all, func(i, j int) bool {
		return all[i].Timestamp.Before(all[j].Timestamp)
	})

	var out []Message
	for _, m := range all {
		if conversationID != "" && m.ConversationID != conversationID {
			continue
		}
		out = append(out, m)
		if limit > 0 && int64(len(out)) >= limit {
			break
		}
	}
	return out, nil
}

// Len returns the total number of stored messages.
func (s *MemStore) Len() int {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return len(s.messages)
}
