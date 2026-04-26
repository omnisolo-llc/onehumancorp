package builtin

import (
    "context"
    "encoding/json"
)

// Stub for broken dependency references
type Agent struct{}

type LLMClient interface {
    Chat(ctx context.Context, req ChatRequest) (*ChatResponse, error)
}
type ChatRequest struct {
    System string
    Messages []Message
    MaxTokens int
}
type Message struct {
    Role string
    Content string
}
type ChatResponse struct {
    Content string
    Message Message
}


type Tool struct {
    Name string
    Description string
    Parameters json.RawMessage
    Execute func(ctx context.Context, args json.RawMessage) (string, error)
}

const RoleUser = "user"

func NewAnthropicClient(key string) LLMClient { return nil }
func NewOpenAIClient(key string) LLMClient { return nil }
func NewOllamaClient(url string) LLMClient { return nil }
