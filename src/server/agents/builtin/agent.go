package builtin

import (
	"context"
	"encoding/json"
)

type LLMClient interface {
	Chat(ctx context.Context, req ChatRequest) (ChatResponse, error)
}

type ChatRequest struct {
	System    string
	Messages  []Message
	MaxTokens int
}

type Message struct {
	Role    string
	Content string
}

type ChatResponse struct {
	Message Message
}

const RoleUser = "user"

type Tool struct {
	Name        string
	Description string
	Parameters  json.RawMessage
}
