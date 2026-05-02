package registry

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"sync"
	"time"
)

// ToolManifest represents the metadata and schema of a registered tool.
type ToolManifest struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// AgentTool defines the interface that all tools must implement.
type AgentTool interface {
	Name() string
	Description() string
	InputSchema() json.RawMessage
	Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error)
}

// UnifiedToolRegistry manages tool registration and discovery.
type UnifiedToolRegistry struct {
	mu    sync.RWMutex
	tools map[string]AgentTool
}

// NewUnifiedToolRegistry creates a new instance of UnifiedToolRegistry.
func NewUnifiedToolRegistry() *UnifiedToolRegistry {
	return &UnifiedToolRegistry{
		tools: make(map[string]AgentTool),
	}
}

// RegisterTool registers a tool with the registry. It validates the input schema.
func (r *UnifiedToolRegistry) RegisterTool(tool AgentTool) error {
	r.mu.Lock()
	defer r.mu.Unlock()

	name := tool.Name()
	if name == "" {
		return fmt.Errorf("tool name cannot be empty")
	}

	// Basic schema validation: ensure it's valid JSON
	schema := tool.InputSchema()
	if len(schema) == 0 {
		return fmt.Errorf("tool input schema cannot be empty")
	}
	if !json.Valid(schema) {
		return fmt.Errorf("tool input schema is not valid JSON")
	}

	r.tools[name] = tool
	return nil
}

// ListTools returns a list of manifests for all registered tools.
func (r *UnifiedToolRegistry) ListTools() []ToolManifest {
	r.mu.RLock()
	defer r.mu.RUnlock()

	manifests := make([]ToolManifest, 0, len(r.tools))
	for _, tool := range r.tools {
		manifests = append(manifests, ToolManifest{
			Name:        tool.Name(),
			Description: tool.Description(),
			InputSchema: tool.InputSchema(),
		})
	}
	return manifests
}

// GetTool retrieves a tool by name.
func (r *UnifiedToolRegistry) GetTool(name string) (AgentTool, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()
	tool, exists := r.tools[name]
	return tool, exists
}

// ExecuteTool wraps the underlying tool execution with telemetry and logging.
func (r *UnifiedToolRegistry) ExecuteTool(ctx context.Context, name string, input json.RawMessage) (json.RawMessage, error) {
	tool, exists := r.GetTool(name)
	if !exists {
		return nil, fmt.Errorf("tool %q not found", name)
	}

	start := time.Now()
	log.Printf("Executing tool %q", name)

	// Delegate execution to the registered tool
	output, err := tool.Execute(ctx, input)

	duration := time.Since(start)
	if err != nil {
		log.Printf("Tool %q failed after %v: %v", name, duration, err)
	} else {
		log.Printf("Tool %q completed successfully in %v", name, duration)
	}

	return output, err
}
