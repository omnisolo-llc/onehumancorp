package sandbox

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

// ShellSession tracks state across executions
type ShellSession struct {
	SessionID  string
	SandboxDir string
	CurrentCwd string
}

// NewShellSession creates a new ShellSession
func NewShellSession(sessionID, sandboxDir string) (*ShellSession, error) {
	// Initialize sandbox directory
	if err := os.MkdirAll(sandboxDir, 0755); err != nil {
		return nil, fmt.Errorf("failed to create sandbox dir: %w", err)
	}

	// Create an empty env_snapshot.sh
	envSnapshotPath := filepath.Join(sandboxDir, "env_snapshot.sh")
	if _, err := os.Stat(envSnapshotPath); os.IsNotExist(err) {
		if err := os.WriteFile(envSnapshotPath, []byte(""), 0644); err != nil {
			return nil, fmt.Errorf("failed to create empty env_snapshot.sh: %w", err)
		}
	}

	// CurrentCwd defaults to sandboxDir
	return &ShellSession{
		SessionID:  sessionID,
		SandboxDir: sandboxDir,
		CurrentCwd: sandboxDir,
	}, nil
}

// RunStatefulCommand runs a command in a stateful shell
func (s *ShellSession) RunStatefulCommand(ctx context.Context, cmdStr string) (string, error) {
	envSnapshotPath := filepath.Join(s.SandboxDir, "env_snapshot.sh")
	cwdSnapshotPath := filepath.Join(s.SandboxDir, "cwd_snapshot.txt")

	// Avoid exposing parent environment to the sandbox by emptying the environment,
	// except for critical ones if needed, or starting with a blank slate that only
	// loads the snapshot. We can do this by setting cmd.Env.

	wrapperCmd := fmt.Sprintf(`source '%s' 2>/dev/null || true; { %s; }; declare -p > '%s'; pwd -P > '%s'`,
		envSnapshotPath, cmdStr, envSnapshotPath, cwdSnapshotPath)

	cmd := exec.CommandContext(ctx, "bash", "-c", wrapperCmd)
	cmd.Dir = s.CurrentCwd
	// Pass an empty environment so that declare -p doesn't capture the server's environment
	// including secrets like GITHUB_TOKEN
	cmd.Env = []string{"PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"}

	out, err := cmd.CombinedOutput()

	// Try to update current working directory if it succeeded
	if cwdBytes, err := os.ReadFile(cwdSnapshotPath); err == nil {
		cwd := strings.TrimSpace(string(cwdBytes))
		if cwd != "" {
			s.CurrentCwd = cwd
		}
	}

	return string(out), err
}
