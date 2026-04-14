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

// AllTools returns the full set of builtin tools available to general-purpose agents.
// Mirrors CC-Source's ASYNC_AGENT_ALLOWED_TOOLS + IN_PROCESS_TEAMMATE_ALLOWED_TOOLS.
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
		TodoReadTool,
		ToolSearchTool,
		TaskCreateTool,
		TaskGetTool,
		TaskListTool,
		TaskUpdateTool,
		SleepTool,
	}
}

// CoordinatorTools returns the restricted tool set for coordinator agents.
// Mirrors CC-Source's COORDINATOR_MODE_ALLOWED_TOOLS.
// The coordinator only has tools to spawn agents, stop agents, and send messages.
func CoordinatorTools() []Tool {
	return []Tool{
		AgentTool,
		TaskStopTool,
		TaskStatusTool,
		SendMessageTool,
	}
}

// AgentToolsWithSubagentSupport returns the tool set for agents that can spawn sub-agents.
// Includes everything from AllTools plus the agent management tools.
func AgentToolsWithSubagentSupport() []Tool {
	base := AllTools()
	return append(base,
		AgentTool,
		TaskStopTool,
		TaskStatusTool,
	)
}
