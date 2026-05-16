//go:build !linux && !darwin

package harness

import (
	"context"
	"fmt"
)

// UnsupportedSandboxManager implements SandboxManager for unsupported OSs.
type UnsupportedSandboxManager struct{}

// NewSandboxManager returns an error-producing SandboxManager on unsupported platforms.
func NewSandboxManager(workspaceDir string) SandboxManager {
	return &UnsupportedSandboxManager{}
}

// ExecuteCommand always fails on unsupported platforms.
func (sm *UnsupportedSandboxManager) ExecuteCommand(ctx context.Context, command string, args ...string) (string, string, error) {
	return "", "", fmt.Errorf("sandbox is not supported on this operating system")
}
