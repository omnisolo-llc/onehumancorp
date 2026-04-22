package registry

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"sync"

	"github.com/santhosh-tekuri/jsonschema/v5"
)

// AgentTool defines the interface for a strongly typed tool
type AgentTool interface {
	Name() string
	Description() string
	InputSchema() json.RawMessage
	Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error)
}

// ToolManifest provides discovery information about a tool
type ToolManifest struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"input_schema"`
}

// UnifiedToolRegistry manages a collection of strongly typed AgentTools
type UnifiedToolRegistry struct {
	mu    sync.RWMutex
	tools map[string]AgentTool
}

// NewUnifiedToolRegistry creates a new instance of UnifiedToolRegistry
func NewUnifiedToolRegistry() *UnifiedToolRegistry {
	return &UnifiedToolRegistry{
		tools: make(map[string]AgentTool),
	}
}

// RegisterTool registers an AgentTool after validating its InputSchema
func (r *UnifiedToolRegistry) RegisterTool(tool AgentTool) error {
	schema := tool.InputSchema()

	compiler := jsonschema.NewCompiler()
	if err := compiler.AddResource("schema.json", bytes.NewReader(schema)); err != nil {
		return fmt.Errorf("invalid json schema for tool %q: %w", tool.Name(), err)
	}

	_, err := compiler.Compile("schema.json")
	if err != nil {
		return fmt.Errorf("invalid json schema for tool %q: %w", tool.Name(), err)
	}

	r.mu.Lock()
	defer r.mu.Unlock()

	if _, exists := r.tools[tool.Name()]; exists {
		return fmt.Errorf("tool %q is already registered", tool.Name())
	}

	r.tools[tool.Name()] = tool
	return nil
}

// ListTools returns a list of ToolManifests for all registered tools
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

// GetTool retrieves a tool by name
func (r *UnifiedToolRegistry) GetTool(name string) (AgentTool, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	tool, exists := r.tools[name]
	return tool, exists
}

// Execute intercepts the Execute call to provide standard metrics/logging boundary
func (r *UnifiedToolRegistry) Execute(ctx context.Context, name string, input json.RawMessage) (json.RawMessage, error) {
	tool, exists := r.GetTool(name)
	if !exists {
		return nil, fmt.Errorf("tool %q not found", name)
	}

	// Here we could add telemetry, logging, metrics boundary

	return tool.Execute(ctx, input)
}
