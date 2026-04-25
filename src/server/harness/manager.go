package harness

import (
	"context"
	"io"
	"os"
	"time"

	"github.com/onehumancorp/mono/src/server/telemetry"
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
	// ExecuteStream runs a command within a sandboxed environment and bridges streaming I/O.
	ExecuteStream(ctx context.Context, command string, policy *Policy, stdin io.Reader, stdout, stderr io.Writer, agentID, orgID, role, model string) (Result, error)
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

	start := time.Now()
	// 2. Execute via runner (which performs its own AST validation)
	res, err := m.runner.ExecuteWithPolicy(ctx, wrapped, policy)

	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordHarnessExecutionLatency(ctx, float64(time.Since(start).Milliseconds()), mode)

	return res, err
}

// ExecuteStream runs a command within a sandboxed environment and bridges streaming I/O.
func (m *Manager) ExecuteStream(ctx context.Context, command string, policy *Policy, stdin io.Reader, stdout, stderr io.Writer, agentID, orgID, role, model string) (Result, error) {
	if policy == nil {
		policy = &m.config.DefaultPolicy
	}

	// 1. Wrap command
	wrapped, err := m.WrapCommand(ctx, command, policy)
	if err != nil {
		return Result{}, err
	}

	// Calculate and record cost (simplified simulation based on command length)
	cost := float64(len(command)) * 0.0001
	telemetry.RecordAgentCost(ctx, agentID, orgID, role, model, cost)

	start := time.Now()
	// 2. Execute via runner with streaming I/O (assuming runner has an equivalent streaming method)
	// Currently bridging to standard ExecuteWithPolicy, but modifying writer if implemented fully
	// Wait, we need to add ExecuteStream to runner too. For now we use the existing method.
	res, err := m.runner.ExecuteStream(ctx, wrapped, policy, stdin, stdout, stderr)

	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordHarnessExecutionLatency(ctx, float64(time.Since(start).Milliseconds()), mode)

	return res, err
}
