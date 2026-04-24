//go:build linux || darwin

package harness

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"syscall"

	"github.com/google/uuid"
)

// LocalShellTask represents an isolated local shell process tracked by ID,
// supporting explicit cleanup of output directories and strict process group
// termination for safe execution inside the Agent Harness.
type LocalShellTask struct {
	ID      string
	TempDir string
	Cmd     *exec.Cmd
}

// NewLocalShellTask creates a new LocalShellTask with a unique ID and process group isolation.
func NewLocalShellTask(ctx context.Context, command string, args ...string) *LocalShellTask {
	taskID := uuid.New().String()

	cmd := exec.CommandContext(ctx, command, args...)
	// Setpgid creates a new process group for the command,
	// making it possible to kill the entire process tree using the negative PID.
	cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true}

	task := &LocalShellTask{
		ID:      taskID,
		TempDir: filepath.Join(os.TempDir(), "ohc_task_"+taskID),
		Cmd:     cmd,
	}

	// Ensure the entire process group is killed when the context is canceled.
	cmd.Cancel = func() error {
		return task.Kill()
	}

	return task
}

// Start initiates the command but does not wait for it to complete.
func (t *LocalShellTask) Start() error {
	if t.TempDir != "" {
		if err := os.MkdirAll(t.TempDir, 0755); err != nil {
			return fmt.Errorf("failed to create task temp dir: %w", err)
		}
	}
	return t.Cmd.Start()
}

// Run starts the command and waits for it to complete.
func (t *LocalShellTask) Run() error {
	if t.TempDir != "" {
		if err := os.MkdirAll(t.TempDir, 0755); err != nil {
			return fmt.Errorf("failed to create task temp dir: %w", err)
		}
	}
	return t.Cmd.Run()
}

// Kill explicitly terminates the command's process group and removes its temporary directory.
func (t *LocalShellTask) Kill() error {
	var killErr error

	if t.Cmd.Process != nil {
		// Send SIGKILL to the process group (-pid)
		// This ensures that all child processes spawned by this command are also killed.
		pgid := -t.Cmd.Process.Pid
		killErr = syscall.Kill(pgid, syscall.SIGKILL)
		if killErr != nil && killErr == syscall.ESRCH {
			// Process group no longer exists. Safe to ignore.
			killErr = nil
		}
	}

	var rmErr error
	if t.TempDir != "" {
		rmErr = os.RemoveAll(t.TempDir)
	}

	if killErr != nil {
		return fmt.Errorf("failed to kill process group: %w", killErr)
	}
	if rmErr != nil {
		return fmt.Errorf("failed to remove task temp dir: %w", rmErr)
	}

	return nil
}
