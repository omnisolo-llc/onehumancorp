package builtin

import (
	"context"
	"os"
)

// BuiltinAgent handles the core loop for the builtin agent.
type BuiltinAgent struct {
	Client      LLMClient
	Model       string
	System      string
	Tools       []Tool
	MaxTokens   int
	Temperature float32

	// Context and Fallback configs inspired by Claude Code harness
	FallbackModel     string
	MaxContextTokens  int
	MaxOutputEscalate int
	UseStreaming      bool
	ToolPermission    string // e.g. "plan" vs "execute"
	AllowedTools      []string
	DeniedTools       []string
}

// LLMClient is the interface for talking to the LLM backend.
type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (ChatResponse, error)
	ChatStream(ctx context.Context, req ChatRequest, chunkChan chan<- ChatResponseChunk) error
}

// Initialize sets up the session lifecycle.
func (a *BuiltinAgent) Initialize(ctx context.Context) error {
	// Initialize tools, env, etc.
	if len(a.AllowedTools) == 0 && os.Getenv("OHC_ALLOWED_TOOLS") != "" {
		a.AllowedTools = []string{os.Getenv("OHC_ALLOWED_TOOLS")}
	}
	if a.System == "" {
		a.System = "You are the default OHC builtin agent. Be direct and autonomous."
	}
	return nil
}

// Teardown cleans up the session lifecycle.
func (a *BuiltinAgent) Teardown(ctx context.Context) {
	// Cleanup background processes, streams, etc.
	a.Tools = nil
}
