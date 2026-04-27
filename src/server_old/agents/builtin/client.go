package builtin

import "context"

type Role string
const (
    RoleUser Role = "user"
)

type Message struct {
    Role Role
    Content string
}

type ChatRequest struct {
    System string
    Messages []Message
    MaxTokens int
}

type ChatResponse struct {
    Message Message
}

type LLMClient interface {
    Chat(ctx context.Context, req ChatRequest) (ChatResponse, error)
}

type AnthropicClient struct{}
func NewAnthropicClient(key string) *AnthropicClient { return &AnthropicClient{} }
func (c *AnthropicClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) { return ChatResponse{}, nil }

type OpenAIClient struct{}
func NewOpenAIClient(key string) *OpenAIClient { return &OpenAIClient{} }
func (c *OpenAIClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) { return ChatResponse{}, nil }

type OllamaClient struct{}
func NewOllamaClient(key string) *OllamaClient { return &OllamaClient{} }
func (c *OllamaClient) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) { return ChatResponse{}, nil }
