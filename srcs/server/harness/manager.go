package harness

import (
	"context"
)

// Policy defines the granular security constraints for a sandbox execution.
type Policy struct {
	AllowedPaths  []string `json:"allowedPaths"`
	ReadOnlyPaths []string `json:"readOnlyPaths"`
	BlockedPaths  []string `json:"blockedPaths"`
	AllowedHosts  []string `json:"allowedHosts"`
	AllowNetwork  bool     `json:"allowNetwork"`
}

// Config defines the overall configuration for the SandboxManager.
type Config struct {
	DefaultPolicy Policy `json:"defaultPolicy"`
}

// SandboxManager defines the interface for managing isolated execution contexts.
type SandboxManager interface {
	AgentHarness

	// Initialize sets up the sandbox environment.
	Initialize(ctx context.Context) error

	// UpdateConfig updates the manager's configuration.
	UpdateConfig(ctx context.Context, config Config) error

	// WrapCommand transforms a command according to the policy (if needed).
	WrapCommand(ctx context.Context, command string, policy *Policy) (string, error)

	// ExecuteWithPolicy runs a command within a sandboxed environment governed by the provided policy.
	ExecuteWithPolicy(ctx context.Context, command string, policy *Policy) (Result, error)
}

// Manager is the concrete implementation of SandboxManager.
type Manager struct {
	config    Config
	validator *ASTValidator
	runner    *BwrapRunner
}

// NewManager creates a new SandboxManager implementation.
func NewManager(validator *ASTValidator, runner *BwrapRunner) *Manager {
	if validator == nil {
		validator = NewASTValidator()
	}
	if runner == nil {
		runner = NewBwrapRunner(validator)
	}
	return &Manager{
		validator: validator,
		runner:    runner,
	}
}

// Initialize sets up the sandbox environment.
func (m *Manager) Initialize(ctx context.Context) error {
	// In the future, this could ensure bwrap is installed or pre-mount certain filesystems.
	return nil
}

// UpdateConfig updates the manager's configuration.
func (m *Manager) UpdateConfig(ctx context.Context, config Config) error {
	m.config = config
	return nil
}

// WrapCommand transforms a command according to the policy.
func (m *Manager) WrapCommand(ctx context.Context, command string, policy *Policy) (string, error) {
	// Placeholder for future command transformations (e.g. injecting environment variables or wrappers)
	return command, nil
}

// Execute runs a command using the default policy.
func (m *Manager) Execute(ctx context.Context, command string) (Result, error) {
	return m.ExecuteWithPolicy(ctx, command, nil)
}

// ExecuteWithPolicy runs a command within a sandboxed environment governed by the provided policy.
func (m *Manager) ExecuteWithPolicy(ctx context.Context, command string, policy *Policy) (Result, error) {
	if policy == nil {
		policy = &m.config.DefaultPolicy
	}

	// 1. Wrap command
	wrapped, err := m.WrapCommand(ctx, command, policy)
	if err != nil {
		return Result{}, err
	}

	// 2. Execute via runner (which performs its own AST validation)
	return m.runner.ExecuteWithPolicy(ctx, wrapped, policy)
}
