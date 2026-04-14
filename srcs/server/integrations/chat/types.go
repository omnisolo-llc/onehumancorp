package chat

import (
	"context"
	"time"
)

// Message is a persisted chat payload for replay and debugging.
type Message struct {
	ID             string
	ConversationID string
	Channel        string
	Sender         string
	Text           string
	Timestamp      time.Time
}

// Store persists chat messages and supports replay.
type Store interface {
	Append(ctx context.Context, msg Message) (string, error)
	Replay(ctx context.Context, conversationID string, limit int64) ([]Message, error)
}
