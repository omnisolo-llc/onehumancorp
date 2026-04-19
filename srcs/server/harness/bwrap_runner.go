package harness

import (
	"bytes"
	"context"
	"fmt"
	"os/exec"
)

// BwrapRunner executes commands inside a bubblewrap sandbox.
type BwrapRunner struct {
	validator *ASTValidator
}

// NewBwrapRunner creates a new BwrapRunner.
func NewBwrapRunner(validator *ASTValidator) *BwrapRunner {
	if validator == nil {
		validator = NewASTValidator()
	}
	return &BwrapRunner{
		validator: validator,
	}
}

// GetBwrapArgs generates the bwrap arguments for a given command.
// We extract this to make it testable.
func (r *BwrapRunner) GetBwrapArgs(command string) []string {
	return []string{
		"--unshare-net",
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--ro-bind", "/", "/",
		"--proc", "/proc",
		"--dev", "/dev",
		"--tmpfs", "/tmp",
        // Using socat Unix Socket proxy to strictly control network egress
        "--bind", "/var/run/ohc_proxy.sock", "/var/run/ohc_proxy.sock",
		"--",
		"bash", "-c", command,
	}
}

// Execute runs the command in a bwrap sandbox after AST validation.
func (r *BwrapRunner) Execute(ctx context.Context, command string) (Result, error) {
	if err := r.validator.Validate(ctx, command); err != nil {
		return Result{}, fmt.Errorf("command validation failed: %w", err)
	}

	bwrapArgs := r.GetBwrapArgs(command)
	cmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()
	exitCode := 0
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			exitCode = exitError.ExitCode()
            // When bwrap policy fails or bwrap itself fails to set up the sandbox
            // we should count that as a violation. If it's just bash failing, we shouldn't.
            // Bwrap often exits with 1 if it fails to set up, or passes through the exit code.
            // We'll treat exit code 1 as a potential setup violation since we can't easily distinguish
            // unless we parse stderr.
            if exitCode == 1 && bytes.Contains(stderr.Bytes(), []byte("bwrap:")) {
                violationCount.Add(ctx, 1)
            }
		} else {
            // Infrastructure error launching bwrap process
			return Result{}, fmt.Errorf("failed to run bwrap: %w", err)
		}
	}

	return Result{
		Stdout:   stdout.String(),
		Stderr:   stderr.String(),
		ExitCode: exitCode,
	}, nil
}
