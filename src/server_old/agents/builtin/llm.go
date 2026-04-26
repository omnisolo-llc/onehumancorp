package builtin

import "context"

type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (*ChatResponse, error)
}

func NewAnthropicClient(key string) LLMClient {
	return nil
}

func NewOpenAIClient(key string) LLMClient {
	return nil
}

func NewOllamaClient(key string) LLMClient {
	return nil
}
