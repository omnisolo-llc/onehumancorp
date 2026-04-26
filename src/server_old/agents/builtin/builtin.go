package builtin

import (
	"context"
	"encoding/json"
)

type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	Parameters  json.RawMessage `json:"parameters"`
	Execute     func(ctx context.Context, input json.RawMessage) (string, error)
}

type Message struct {
	Role    string
	Content string
}

type ChatRequest struct {
	System    string
	Messages  []Message
	MaxTokens int
}

type ChatResponse struct {
	Message Message
}

type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (*ChatResponse, error)
}

const (
	RoleUser      = "user"
	RoleAssistant = "assistant"
	RoleSystem    = "system"
)

func NewAnthropicClient(apiKey string) LLMClient {
	return nil
}

func NewOpenAIClient(apiKey string) LLMClient {
	return nil
}

func NewOllamaClient(endpoint string) LLMClient {
	return nil
}
