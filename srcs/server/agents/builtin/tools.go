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

// AllTools returns the full set of builtin tools.
func AllTools() []Tool {
	return []Tool{
		BashTool,
		FileReadTool,
		FileWriteTool,
		FileEditTool,
		GlobTool,
		GrepTool,
		WebFetchTool,
		WebSearchTool,
		SendMessageTool,
		TodoWriteTool,
		ToolSearchTool,
		TaskCreateTool,
		TaskGetTool,
		TaskListTool,
		TaskUpdateTool,
	}
}
