package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
)

// Tool represents an executable function the agent can call.
type Tool struct {
	Name        string
	Description string
	Parameters  json.RawMessage // JSON Schema of parameters
	Execute     func(ctx context.Context, args json.RawMessage) (string, error)
}

// ToolPermissionContext defines permission policies.
// Mirrors CC-Source's tool permission behavior.
type ToolPermissionContext struct {
	mu           sync.RWMutex
	AllowedTools map[string]bool
	DeniedTools  map[string]bool
	// AutoMode implies tools are allowed without interactive checks unless explicitly denied.
	AutoMode bool
}

// NewToolPermissionContext creates a context where all builtin tools are allowed by default if autoMode is true.
func NewToolPermissionContext(autoMode bool) *ToolPermissionContext {
	return &ToolPermissionContext{
		AllowedTools: make(map[string]bool),
		DeniedTools:  make(map[string]bool),
		AutoMode:     autoMode,
	}
}

// CanExecute checks if the tool is permitted to run.
func (p *ToolPermissionContext) CanExecute(toolName string) error {
	p.mu.RLock()
	defer p.mu.RUnlock()

	if p.DeniedTools[toolName] {
		return fmt.Errorf("tool %q is explicitly denied by policy", toolName)
	}

	if p.AllowedTools[toolName] {
		return nil
	}

	if !p.AutoMode {
		return fmt.Errorf("tool %q requires interactive permission (auto-mode disabled)", toolName)
	}

	return nil
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
