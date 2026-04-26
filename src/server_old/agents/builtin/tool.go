package builtin

import (
	"context"
	"encoding/json"
)

type Tool struct {
	Name        string
	Description string
	Parameters  json.RawMessage
	Execute     func(ctx context.Context, args json.RawMessage) (string, error)
}

type ChatRequest struct {
	Model       string
	System      string
	Messages    []Message
	Tools       []Tool
	MaxTokens   int
	Temperature float32
}

type ChatResponse struct {
	Message    Message
	Usage      Usage
	StopReason string
}

type Usage struct {
	InputTokens  int
	OutputTokens int
}

type Message struct {
	Role    string
	Content string
}

const (
	RoleUser      = "user"
	RoleAssistant = "assistant"
	RoleSystem    = "system"
	RoleTool      = "tool"
)

type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (ChatResponse, error)
}

func NewAnthropicClient(apiKey string) LLMClient { return nil }
func NewOpenAIClient(apiKey string) LLMClient { return nil }
func NewOllamaClient(url string) LLMClient { return nil }
func NewGeminiClient(apiKey string) LLMClient { return nil }
