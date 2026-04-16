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

	// ToolPermissions enforces allow/deny policies for tool execution.
	// If nil, all tools are permitted without interactive prompting (auto-mode).
	ToolPermissions *ToolPermissionContext
}

// LLMClient is the interface for talking to the LLM backend.
type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (ChatResponse, error)
}

// StreamingLLMClient extends LLMClient to optionally support stream-based responses.
// Mirrors the streaming requirement of the harness to stream incremental chunks back to UI.
type StreamingLLMClient interface {
	LLMClient
	ChatStream(ctx context.Context, req ChatRequest, chunkCb func(chunk string)) (ChatResponse, error)
}
