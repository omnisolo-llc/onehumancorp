package harness

import (
	"context"
	"fmt"
	"sync"
)

// Result represents the outcome of an agent execution.
type Result struct {
	Stdout   string
	Stderr   string
	ExitCode int
}

// AgentHarness defines the interface for executing agent commands.
type AgentHarness interface {
	Execute(ctx context.Context, command string) (Result, error)
}

// Registry manages the available agent execution strategies.
type Registry struct {
	mu       sync.RWMutex
	harnesses map[string]AgentHarness
}

// NewRegistry creates a new AgentHarness registry.
func NewRegistry() *Registry {
	return &Registry{
		harnesses: make(map[string]AgentHarness),
	}
}

// Register adds a new harness to the registry.
func (r *Registry) Register(name string, harness AgentHarness) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.harnesses[name] = harness
}

// Get retrieves a harness by name.
func (r *Registry) Get(name string) (AgentHarness, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()
	harness, ok := r.harnesses[name]
	if !ok {
		return nil, fmt.Errorf("harness %q not found", name)
	}
	return harness, nil
}

// GetManager retrieves a harness as a SandboxManager if possible.
func (r *Registry) GetManager(name string) (SandboxManager, error) {
	h, err := r.Get(name)
	if err != nil {
		return nil, err
	}
	manager, ok := h.(SandboxManager)
	if !ok {
		return nil, fmt.Errorf("harness %q is not a SandboxManager", name)
	}
	return manager, nil
}
