package sandbox

import (
	"context"
	"fmt"
	"os"
	"os/exec"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	sandboxViolations = promauto.NewCounter(prometheus.CounterOpts{
		Name: "ohc_sandbox_violations_total",
		Help: "The total number of sandbox violations across all agent sessions",
	})
)

// RecordViolation increments the sandbox violations counter
func RecordViolation() {
	sandboxViolations.Inc()
}

// SandboxManager manages a secure temp directory
type SandboxManager struct {
	SandboxDir string
	Provider   ShellProvider
}

// NewSandboxManager initializes a secure temporary directory per task.
func NewSandboxManager(sessionID string) (*SandboxManager, error) {
	// Use os.MkdirTemp for a secure temp directory with 0700 permissions
	dir, err := os.MkdirTemp("", fmt.Sprintf("ohc-agent-session-%s-*", sessionID))
	if err != nil {
		return nil, err
	}

	// Double check the permissions are strictly 0700
	if err := os.Chmod(dir, 0700); err != nil {
		os.RemoveAll(dir)
		return nil, err
	}

	return &SandboxManager{
		SandboxDir: dir,
		Provider:   &BashProvider{}, // default
	}, nil
}

func (s *SandboxManager) SetProvider(provider ShellProvider) {
	s.Provider = provider
}

func (s *SandboxManager) Execute(ctx context.Context, cmdStr string) (string, error) {
	if s.Provider == nil {
		s.Provider = &BashProvider{}
	}
	return s.Provider.Execute(ctx, s.SandboxDir, cmdStr)
}

func (s *SandboxManager) Cleanup() error {
	return os.RemoveAll(s.SandboxDir)
}

// ShellProvider is an interface that prefixes execution with security flags
type ShellProvider interface {
	Execute(ctx context.Context, sandboxDir string, cmdStr string) (string, error)
}

// getEnv returns the environment variables with TMPDIR set to sandboxDir
func getEnv(sandboxDir string) []string {
	env := os.Environ()
	var filtered []string
	for _, e := range env {
		if len(e) > 7 && e[:7] == "TMPDIR=" {
			continue // skip existing TMPDIR
		}
		filtered = append(filtered, e)
	}
	filtered = append(filtered, fmt.Sprintf("TMPDIR=%s", sandboxDir))
	return filtered
}

// BashProvider implements ShellProvider for Bash
type BashProvider struct{}

func (b *BashProvider) Execute(ctx context.Context, sandboxDir string, cmdStr string) (string, error) {
	wrapperCmd := fmt.Sprintf("shopt -u extglob 2>/dev/null || true; %s", cmdStr)
	cmd := exec.CommandContext(ctx, "bash", "-c", wrapperCmd)
	cmd.Dir = sandboxDir
	cmd.Env = getEnv(sandboxDir)

	out, err := cmd.CombinedOutput()
	return string(out), err
}

// PowerShellProvider implements ShellProvider for PowerShell
type PowerShellProvider struct{}

func (p *PowerShellProvider) Execute(ctx context.Context, sandboxDir string, cmdStr string) (string, error) {
	wrapperCmd := fmt.Sprintf("& { %s }", cmdStr)
	cmd := exec.CommandContext(ctx, "pwsh", "-NoProfile", "-NonInteractive", "-Command", wrapperCmd)
	cmd.Dir = sandboxDir
	cmd.Env = getEnv(sandboxDir)

	out, err := cmd.CombinedOutput()
	return string(out), err
}
