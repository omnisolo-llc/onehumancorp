package harness

import (
	"context"
	"runtime"
	"time"

	"onehumancorp/srcs/server/telemetry"
)

// SandboxManager interface defines the contract for Agent Harness execution wrappers
type SandboxManager interface {
	WrapCommand(cmd string) (string, error)
	UpdateConfig(policyJSON string) error
	AnnotateError(err error, stdout string) string
	Execute(cmd string) (string, error)
}

// OS sandbox types to implement the SandboxManager interface directly
type bwrapSandbox struct {
	*BwrapSandboxManager
}

func (s *bwrapSandbox) Execute(cmd string) (string, error) {
	return s.BwrapSandboxManager.Execute(cmd)
}

type macosSandbox struct {
	*MacOSSandboxManager
}

func (s *macosSandbox) Execute(cmd string) (string, error) {
	return s.MacOSSandboxManager.Execute(cmd)
}

// AssistantSandboxManager wraps command execution by delegating to the appropriate OS-level sandbox
type AssistantSandboxManager struct {
	adapter SandboxManager
}

// NewAssistantSandboxManager creates a new AssistantSandboxManager based on the host OS
func NewAssistantSandboxManager() *AssistantSandboxManager {
	var adapter SandboxManager
	switch runtime.GOOS {
	case "darwin":
		adapter = &macosSandbox{NewMacOSSandboxManager()}
	case "linux":
		adapter = &bwrapSandbox{NewBwrapSandboxManager()}
	default:
		// Fallback for WSL2 or unsupported OSes could be added here
		// For now, default to bwrap assuming a Linux-like environment
		adapter = &bwrapSandbox{NewBwrapSandboxManager()}
	}

	return &AssistantSandboxManager{
		adapter: adapter,
	}
}

// WrapCommand implements SandboxManager.WrapCommand
func (m *AssistantSandboxManager) WrapCommand(cmd string) (string, error) {
	start := time.Now()
	res, err := m.adapter.WrapCommand(cmd)
	telemetry.RecordHarnessInitLatency(context.Background(), time.Since(start).Seconds())
	return res, err
}

// UpdateConfig implements SandboxManager.UpdateConfig
func (m *AssistantSandboxManager) UpdateConfig(policyJSON string) error {
	return m.adapter.UpdateConfig(policyJSON)
}

// AnnotateError implements SandboxManager.AnnotateError
func (m *AssistantSandboxManager) AnnotateError(err error, stdout string) string {
	return m.adapter.AnnotateError(err, stdout)
}

// Execute implements execution by type casting to appropriate types
func (m *AssistantSandboxManager) Execute(cmd string) (string, error) {
	return m.adapter.Execute(cmd)
}
