package harness

import (
	"context"
	"fmt"
	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/harness/network"
	"path/filepath"
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
	NetworkSocket string `json:"networkSocket"`
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
	bridge    *network.NetworkBridge
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
	if m.config.NetworkSocket == "" {
		m.config.NetworkSocket = filepath.Join("/tmp", fmt.Sprintf("ohc_proxy_%s.sock", uuid.New().String()))
	}
	m.bridge = network.NewNetworkBridge(m.config.NetworkSocket, m.config.DefaultPolicy.AllowedHosts)
	return m.bridge.Start()
}

// UpdateConfig updates the manager's configuration.
func (m *Manager) UpdateConfig(ctx context.Context, config Config) error {
	m.config = config
	return nil
}

// WrapCommand transforms a command according to the policy.
func (m *Manager) WrapCommand(ctx context.Context, command string, policy *Policy) (string, error) {
	if policy != nil && !policy.AllowNetwork && m.config.NetworkSocket != "" {
		proxyURL := "http://127.0.0.1:8080"
		wrapped := fmt.Sprintf(`socat TCP-LISTEN:8080,fork UNIX-CLIENT:%s &
sleep 0.1
export HTTP_PROXY=%s
export HTTPS_PROXY=%s
export ALL_PROXY=%s
%s`, m.config.NetworkSocket, proxyURL, proxyURL, proxyURL, command)
		return wrapped, nil
	}
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
	return m.runner.ExecuteWithPolicy(ctx, wrapped, policy, m.config.NetworkSocket)
}
