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

// ChatStreamResponse represents a chunk of streaming output from the LLM.
type ChatStreamResponse struct {
	Content   string     `json:"content"`
	ToolCalls []ToolCall `json:"tool_calls,omitempty"`
	Done      bool       `json:"done"`
}

// LLMStreamClient is the interface for talking to an LLM backend that supports streaming.
type LLMStreamClient interface {
	ChatStream(ctx context.Context, req ChatRequest, stream func(ChatStreamResponse) error) error
}
