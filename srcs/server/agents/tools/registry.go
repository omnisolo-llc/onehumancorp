package tools

import (
	"context"
	"encoding/json"
	"errors"
	"sync"
)

// Tool represents the contract every tool must implement to be registered.
type Tool interface {
	// Name returns the canonical name of the tool.
	Name() string

	// Description returns a brief explanation of what the tool does.
	Description() string

	// InputSchema returns the JSON Schema defining the expected parameters.
	InputSchema() json.RawMessage

	// Execute runs the tool with the given context and JSON arguments.
	Execute(ctx context.Context, args json.RawMessage) (string, error)
}

// ToolRegistry manages a collection of strongly typed tools.
type ToolRegistry interface {
	// Register adds a tool to the registry.
	Register(t Tool) error

	// Get retrieves a tool by its name.
	Get(name string) (Tool, bool)

	// All returns all registered tools.
	All() []Tool
}

// defaultRegistry is the standard implementation of ToolRegistry.
type defaultRegistry struct {
	mu    sync.RWMutex
	tools map[string]Tool
}

// NewRegistry creates a new, empty ToolRegistry.
func NewRegistry() ToolRegistry {
	return &defaultRegistry{
		tools: make(map[string]Tool),
	}
}

// Register implements ToolRegistry.Register.
func (r *defaultRegistry) Register(t Tool) error {
	if t == nil {
		return errors.New("cannot register nil tool")
	}

	name := t.Name()
	if name == "" {
		return errors.New("tool must have a name")
	}

	r.mu.Lock()
	defer r.mu.Unlock()

	if _, exists := r.tools[name]; exists {
		return errors.New("tool already registered: " + name)
	}

	r.tools[name] = t
	return nil
}

// Get implements ToolRegistry.Get.
func (r *defaultRegistry) Get(name string) (Tool, bool) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	t, exists := r.tools[name]
	return t, exists
}

// All implements ToolRegistry.All.
func (r *defaultRegistry) All() []Tool {
	r.mu.RLock()
	defer r.mu.RUnlock()

	all := make([]Tool, 0, len(r.tools))
	for _, t := range r.tools {
		all = append(all, t)
	}
	return all
}

// LegacyWrapper adapts a legacy builtin.Tool (which is a struct) to the new Tool interface.
// This is useful for incremental migration.
type LegacyWrapper struct {
	NameVal        string
	DescriptionVal string
	ParametersVal  json.RawMessage
	ExecuteFn      func(ctx context.Context, args json.RawMessage) (string, error)
}

func (l *LegacyWrapper) Name() string {
	return l.NameVal
}

func (l *LegacyWrapper) Description() string {
	return l.DescriptionVal
}

func (l *LegacyWrapper) InputSchema() json.RawMessage {
	return l.ParametersVal
}

func (l *LegacyWrapper) Execute(ctx context.Context, args json.RawMessage) (string, error) {
	if l.ExecuteFn == nil {
		return "", errors.New("tool execution function not defined")
	}
	return l.ExecuteFn(ctx, args)
}
