package builtin

import "context"

// BuiltinAgent handles the core loop for the builtin agent.
type BuiltinAgent struct {
	Client      LLMClient
	Model       string
	System      string
	Tools       []Tool
	MaxTokens   int
	Temperature float32
	MaxTaskBudget int // Maximum output tokens permitted for an entire task
}

// LLMClient is the interface for talking to the LLM backend.
type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (ChatResponse, error)
}
