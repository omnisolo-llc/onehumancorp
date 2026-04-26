package builtin

import (
	"context"
	"encoding/json"
)

type Message struct {
	Role    string
	Content string
}

const RoleUser = "user"

type ChatRequest struct {
	System    string
	Messages  []Message
	MaxTokens int
}

type ChatResponse struct {
	Message Message
}

type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (ChatResponse, error)
}

func NewAnthropicClient(key string) LLMClient { return nil }
func NewOpenAIClient(key string) LLMClient { return nil }
func NewOllamaClient(url string) LLMClient { return nil }

type Tool struct {
	Name        string
	Description string
	Parameters  json.RawMessage
	Execute     func(ctx context.Context, args json.RawMessage) (string, error)
}
