package builtin

import (
	"context"
	"encoding/json"
)

// Tool represents an executable function the agent can call.
type Tool struct {
	Name        string
	Description string
	Parameters  json.RawMessage // JSON Schema of parameters
	Execute     func(ctx context.Context, args json.RawMessage) (string, error)
}

// StandardTools is a list of all built-in tools.
var StandardTools = []Tool{
	BashTool,
	FileReadTool,
	FileWriteTool,
	FileEditTool,
	GlobTool,
	GrepTool,
	SendMessageTool,
	TaskCreateTool,
	TaskGetTool,
	TaskListTool,
	TaskUpdateTool,
	TodoWriteTool,
	ToolSearchTool,
	WebFetchTool,
	WebSearchTool,
}
