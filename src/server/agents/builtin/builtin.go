package builtin

import "context"

type Role string
const RoleUser Role = "user"

type ChatRequest struct {
    System string
    Messages []Message
    MaxTokens int
}
type ChatResponse struct {
    Message Message
}

type CompletionRequest struct {}

type Message struct {
	Content string
    Role Role
}

type AssistantMessage struct {
	Message Message
}

type LLMClient interface {
	Chat(context.Context, ChatRequest) (ChatResponse, error)
	Complete(context.Context, CompletionRequest) (*AssistantMessage, error)
}

func NewAnthropicClient(key string) LLMClient { return nil }
func NewOpenAIClient(key string) LLMClient { return nil }
func NewOllamaClient(url string) LLMClient { return nil }

type Provider struct {}

func (p *Provider) Chat(ctx context.Context, req ChatRequest) (ChatResponse, error) {
	return ChatResponse{}, nil
}

func (p *Provider) Complete(ctx context.Context, req CompletionRequest) (*AssistantMessage, error) {
	return &AssistantMessage{}, nil
}
