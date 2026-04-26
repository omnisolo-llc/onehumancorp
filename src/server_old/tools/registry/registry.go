package registry

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
)

// AgentTool defines the standard interface for all tools in the Unified Tool Registry.
type AgentTool interface {
	Name() string
	Description() string
	InputSchema() json.RawMessage
	Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error)
}

// ToolManifest represents the metadata for a registered tool.
type ToolManifest struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// UnifiedToolRegistry manages tool registration and discovery.
type UnifiedToolRegistry struct {
	mu    sync.RWMutex
	tools map[string]AgentTool
}

// NewUnifiedToolRegistry creates a new tool registry.
func NewUnifiedToolRegistry() *UnifiedToolRegistry {
	return &UnifiedToolRegistry{
		tools: make(map[string]AgentTool),
	}
}

// RegisterTool adds a tool to the registry. It validates the schema is valid JSON.
func (r *UnifiedToolRegistry) RegisterTool(tool AgentTool) error {
	schema := tool.InputSchema()
	if len(schema) == 0 {
		return fmt.Errorf("tool '%s' has an empty schema", tool.Name())
	}

	if !json.Valid(schema) {
		return fmt.Errorf("tool '%s' has invalid JSON schema", tool.Name())
	}

	r.mu.Lock()
	defer r.mu.Unlock()

	if _, exists := r.tools[tool.Name()]; exists {
		return fmt.Errorf("tool '%s' is already registered", tool.Name())
	}

	r.tools[tool.Name()] = tool
	return nil
}

// ListTools returns a list of manifests for all registered tools.
func (r *UnifiedToolRegistry) ListTools() []ToolManifest {
	r.mu.RLock()
	defer r.mu.RUnlock()

	var manifests []ToolManifest
	for _, tool := range r.tools {
		manifests = append(manifests, ToolManifest{
			Name:        tool.Name(),
			Description: tool.Description(),
			InputSchema: tool.InputSchema(),
		})
	}
	return manifests
}

// GetTool retrieves a registered tool by name.
func (r *UnifiedToolRegistry) GetTool(name string) (AgentTool, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	tool, exists := r.tools[name]
	return tool, exists
}


// ExecuteTool safely executes a tool by name, handling logging and metrics interception.
func (r *UnifiedToolRegistry) ExecuteTool(ctx context.Context, name string, input json.RawMessage) (json.RawMessage, error) {
	tool, exists := r.GetTool(name)
	if !exists {
		return nil, fmt.Errorf("tool '%s' not found", name)
	}

	// Intercept standard metrics and logging here
	// In a real system, you would use OpenTelemetry or a metrics package.
	// For now, we simulate logging interception at the execution boundary.
	fmt.Printf("[Registry Execution] Executing tool: %s\n", name)

	res, err := tool.Execute(ctx, input)

	if err != nil {
		fmt.Printf("[Registry Execution] Tool %s failed: %v\n", name, err)
	} else {
		fmt.Printf("[Registry Execution] Tool %s executed successfully\n", name)
	}

	return res, err
}
