package builtin

import (
	"context"
	"encoding/json"
)

// Role represents the role of a message sender.
type Role string

const (
	RoleUser      Role = "user"
	RoleAssistant Role = "assistant"
	RoleSystem    Role = "system"
	RoleTool      Role = "tool"
)

// Message represents a message in the chat history.
type Message struct {
	Role        Role         `json:"role"`
	Content     string       `json:"content,omitempty"`
	ToolCalls   []ToolCall   `json:"tool_calls,omitempty"`
	ToolResults []ToolResult `json:"tool_results,omitempty"`
}

// ToolCall represents a request from the assistant to call a tool.
type ToolCall struct {
	ID        string          `json:"id"`
	Name      string          `json:"name"`
	Arguments json.RawMessage `json:"arguments"`
}

// ToolResult represents the output of a tool call.
type ToolResult struct {
	ToolCallID string `json:"tool_call_id"`
	Content    string `json:"content,omitempty"`
	Error      string `json:"error,omitempty"`
}

// ChatRequest is the payload sent to the LLM.
type ChatRequest struct {
	Model       string    `json:"model"`
	System      string    `json:"system,omitempty"`
	Messages    []Message `json:"messages"`
	Tools       []Tool    `json:"tools,omitempty"`
	MaxTokens   int       `json:"max_tokens,omitempty"`
	Temperature float32   `json:"temperature,omitempty"`
}

// ChatResponse is the payload received from the LLM.
type ChatResponse struct {
	Message Message `json:"message"`
}

// TokenUsage tracking
type TokenUsage struct {
	InputTokens  int `json:"input_tokens"`
	OutputTokens int `json:"output_tokens"`
}

// StreamEvent represents a chunk in a streaming response
type StreamEvent struct {
	Type    string  `json:"type"` // e.g. "message_start", "content_block_delta", "message_stop"
	Message Message `json:"message,omitempty"`
	Delta   string  `json:"delta,omitempty"`
	Usage   TokenUsage `json:"usage,omitempty"`
}

// StreamingClient extends LLMClient to support streaming
type StreamingClient interface {
	LLMClient
	ChatStream(ctx context.Context, req ChatRequest) (<-chan StreamEvent, error)
}