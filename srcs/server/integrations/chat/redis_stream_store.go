package chat

import (
	"context"
	"fmt"
	"strconv"
	"time"

	"github.com/redis/go-redis/v9"
)

// RedisStreamStore persists chat messages in Valkey/Redis streams.
type RedisStreamStore struct {
	client   redis.UniversalClient
	stream   string
	maxLen   int64
	blocking time.Duration
}

// NewRedisStreamStore creates a stream-backed chat store.
func NewRedisStreamStore(client redis.UniversalClient, stream string, maxLen int64) *RedisStreamStore {
	if stream == "" {
		stream = "ohc:chat:messages"
	}
	if maxLen <= 0 {
		maxLen = 100000
	}
	return &RedisStreamStore{
		client: client,
		stream: stream,
		maxLen: maxLen,
	}
}

// Append writes one chat message to the stream.
func (s *RedisStreamStore) Append(ctx context.Context, msg Message) (string, error) {
	if s.client == nil {
		return "", fmt.Errorf("redis client is required")
	}
	if msg.Timestamp.IsZero() {
		msg.Timestamp = time.Now().UTC()
	}
	args := &redis.XAddArgs{
		Stream: s.stream,
		MaxLen: s.maxLen,
		Approx: true,
		Values: map[string]interface{}{
			"conversation_id": msg.ConversationID,
			"channel":         msg.Channel,
			"sender":          msg.Sender,
			"text":            msg.Text,
			"timestamp":       msg.Timestamp.UnixMilli(),
		},
	}
	return s.client.XAdd(ctx, args).Result()
}

// Replay returns stored messages for a conversation in insertion order.
func (s *RedisStreamStore) Replay(ctx context.Context, conversationID string, limit int64) ([]Message, error) {
	if s.client == nil {
		return nil, fmt.Errorf("redis client is required")
	}
	if limit <= 0 {
		limit = 500
	}

	entries, err := s.client.XRangeN(ctx, s.stream, "-", "+", limit).Result()
	if err != nil {
		return nil, err
	}

	out := make([]Message, 0, len(entries))
	for _, e := range entries {
		if conversationID != "" && toString(e.Values["conversation_id"]) != conversationID {
			continue
		}

		out = append(out, Message{
			ID:             e.ID,
			ConversationID: toString(e.Values["conversation_id"]),
			Channel:        toString(e.Values["channel"]),
			Sender:         toString(e.Values["sender"]),
			Text:           toString(e.Values["text"]),
			Timestamp:      toTimeMilli(e.Values["timestamp"]),
		})
	}
	return out, nil
}

func toString(v interface{}) string {
	switch t := v.(type) {
	case string:
		return t
	case []byte:
		return string(t)
	default:
		return fmt.Sprint(v)
	}
}

func toTimeMilli(v interface{}) time.Time {
	switch t := v.(type) {
	case int64:
		return time.UnixMilli(t).UTC()
	case int:
		return time.UnixMilli(int64(t)).UTC()
	case string:
		n, err := strconv.ParseInt(t, 10, 64)
		if err != nil {
			return time.Time{}
		}
		return time.UnixMilli(n).UTC()
	case []byte:
		n, err := strconv.ParseInt(string(t), 10, 64)
		if err != nil {
			return time.Time{}
		}
		return time.UnixMilli(n).UTC()
	default:
		return time.Time{}
	}
}
