package builtin

import (
	"github.com/onehumancorp/mono/srcs/server/agents/builtin/tools"
)

// BuiltinAgent handles the core loop for the builtin agent.
type BuiltinAgent struct {
	Client      LLMClient
	Model       string
	System      string
	Tools       []tools.Tool
	MaxTokens   int
	Temperature float32
}

// LLMClient is the interface for talking to the LLM backend.
type LLMClient interface {
	Chat(req ChatRequest) (ChatResponse, error)
}
