package builtin

import "context"

type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (*ChatResponse, error)
}

func NewAnthropicClient(apiKey string) LLMClient { return nil }
func NewOpenAIClient(apiKey string) LLMClient { return nil }
func NewOllamaClient(url string) LLMClient { return nil }
