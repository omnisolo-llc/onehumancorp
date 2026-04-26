package builtin

import (
    "encoding/json"
    "context"
)

type Tool struct {
    Name        string          `json:"name"`
    Description string          `json:"description"`
    Parameters  json.RawMessage `json:"parameters"`
    Execute     func(ctx context.Context, args json.RawMessage) (string, error)
}

type Message struct {
    Role    string `json:"role"`
    Content string `json:"content"`
}

type ChatRequest struct {
    System    string    `json:"system"`
    Messages  []Message `json:"messages"`
    MaxTokens int       `json:"max_tokens"`
}

type ChatResponse struct {
    Message Message `json:"message"`
}

const RoleUser = "user"

type LLMClient interface {
    Chat(ctx context.Context, req ChatRequest) (*ChatResponse, error)
}

func NewAnthropicClient(apiKey string) LLMClient {
    return nil
}

func NewOpenAIClient(apiKey string) LLMClient {
    return nil
}

func NewOllamaClient(url string) LLMClient {
    return nil
}
