package sandbox

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"sync"
	"time"
)

// ShellProvider defines an interface for executing shell commands securely
type ShellProvider interface {
	Execute(ctx context.Context, cmd string) ([]byte, error)
	Cleanup() error
}

// SandboxManager manages a secure isolated environment for executing shell commands.
type SandboxManager struct {
	Dir   string
	mutex sync.Mutex
}

// Cleanup removes the temporary directory created for the sandbox environment
func (sm *SandboxManager) Cleanup() error {
	sm.mutex.Lock()
	defer sm.mutex.Unlock()

	if sm.Dir != "" {
		err := os.RemoveAll(sm.Dir)
		sm.Dir = ""
		return err
	}
	return nil
}

// Execute runs the provided command within the sandbox environment using bash.
func (sm *SandboxManager) Execute(ctx context.Context, cmd string) ([]byte, error) {
	sm.mutex.Lock()
	if sm.Dir == "" {
		dir, err := os.MkdirTemp("", "sandbox")
		if err != nil {
			sm.mutex.Unlock()
			return nil, fmt.Errorf("failed to create sandbox directory: %w", err)
		}
		if err := os.Chmod(dir, 0700); err != nil {
			os.RemoveAll(dir)
			sm.mutex.Unlock()
			return nil, fmt.Errorf("failed to set sandbox directory permissions: %w", err)
		}
		sm.Dir = dir
	}
	sm.mutex.Unlock()

	timeoutCtx, cancel := context.WithTimeout(ctx, 10*time.Minute)
	defer cancel()

	wrappedCmd := "shopt -u extglob 2>/dev/null || true; " + cmd
	c := exec.CommandContext(timeoutCtx, "/bin/bash", "-c", wrappedCmd)

	env := os.Environ()
	var newEnv []string
	for _, e := range env {
		if strings.HasPrefix(e, "TMPDIR=") {
			continue
		}
		newEnv = append(newEnv, e)
	}
	newEnv = append(newEnv, "TMPDIR="+sm.Dir)
	c.Env = newEnv
	c.Dir = sm.Dir

	return c.CombinedOutput()
}

// PowerShellProvider implements the ShellProvider for PowerShell environments (Windows)
type PowerShellProvider struct {
	SandboxManager
}

// Execute runs the provided command within the sandbox environment using powershell.
func (pw *PowerShellProvider) Execute(ctx context.Context, cmd string) ([]byte, error) {
	pw.mutex.Lock()
	if pw.Dir == "" {
		dir, err := os.MkdirTemp("", "sandbox")
		if err != nil {
			pw.mutex.Unlock()
			return nil, fmt.Errorf("failed to create sandbox directory: %w", err)
		}
		if err := os.Chmod(dir, 0700); err != nil {
			os.RemoveAll(dir)
			pw.mutex.Unlock()
			return nil, fmt.Errorf("failed to set sandbox directory permissions: %w", err)
		}
		pw.Dir = dir
	}
	pw.mutex.Unlock()

	timeoutCtx, cancel := context.WithTimeout(ctx, 10*time.Minute)
	defer cancel()

	// Provide a clean execution context for powershell
	wrappedCmd := fmt.Sprintf(`$ErrorActionPreference = 'Stop'; Set-Location -Path '%s'; %s`, pw.Dir, cmd)
	c := exec.CommandContext(timeoutCtx, "powershell", "-NoProfile", "-NonInteractive", "-Command", wrappedCmd)

	env := os.Environ()
	var newEnv []string
	for _, e := range env {
		if strings.HasPrefix(e, "TMPDIR=") || strings.HasPrefix(e, "TEMP=") || strings.HasPrefix(e, "TMP=") {
			continue
		}
		newEnv = append(newEnv, e)
	}
	newEnv = append(newEnv, "TMPDIR="+pw.Dir)
	newEnv = append(newEnv, "TEMP="+pw.Dir)
	newEnv = append(newEnv, "TMP="+pw.Dir)
	c.Env = newEnv
	c.Dir = pw.Dir

	return c.CombinedOutput()
}

// AgentWorker manages the execution environment for AI agents
type AgentWorker struct {
	shell ShellProvider
}

// NewAgentWorker creates a new AgentWorker with a bash shell provider
func NewAgentWorker() *AgentWorker {
	return &AgentWorker{
		shell: &SandboxManager{},
	}
}

// TerminalCall executes a command via the SandboxManager
func (w *AgentWorker) TerminalCall(ctx context.Context, cmd string) ([]byte, error) {
	return w.shell.Execute(ctx, cmd)
}

// Close cleans up resources
func (w *AgentWorker) Close() error {
	return w.shell.Cleanup()
}
