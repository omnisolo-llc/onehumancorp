package harness

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"os/exec"
)

// BwrapRunner executes commands inside a bubblewrap sandbox.
type BwrapRunner struct {
	validator *ASTValidator
	ProxySock string
}

// NewBwrapRunner creates a new BwrapRunner.
func NewBwrapRunner(validator *ASTValidator) *BwrapRunner {
	if validator == nil {
		validator = NewASTValidator()
	}
	return &BwrapRunner{
		validator: validator,
		ProxySock: "/var/run/ohc_proxy.sock",
	}
}

// GetBwrapArgs generates the bwrap arguments for a given command.
// We extract this to make it testable.
func (r *BwrapRunner) GetBwrapArgs(command string, policy *Policy) []string {
	args := []string{
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--proc", "/proc",
		"--dev", "/dev",
		"--tmpfs", "/tmp",
	}

	if policy == nil || !policy.AllowNetwork {
		args = append(args, "--unshare-net")
	}

	// Default read-only bind of root, but specific policies can override this.
	// In a real implementation we would be more restrictive.
	args = append(args, "--ro-bind", "/", "/")

	if policy != nil {
		for _, path := range policy.AllowedPaths {
			args = append(args, "--bind", path, path)
		}
		for _, path := range policy.ReadOnlyPaths {
			args = append(args, "--ro-bind", path, path)
		}
		// BlockedPaths are implemented by mounting an empty tmpfs over them
		for _, path := range policy.BlockedPaths {
			args = append(args, "--tmpfs", path)
		}
	}

	// Using socat Unix Socket proxy to strictly control network egress
	if r.ProxySock != "" {
		args = append(args, "--bind", r.ProxySock, r.ProxySock)
	}
	args = append(args, "--", "bash", "-c", command)

	return args
}

// Execute runs the command in a bwrap sandbox after AST validation.
func (r *BwrapRunner) Execute(ctx context.Context, command string) (Result, error) {
	return r.ExecuteWithPolicy(ctx, command, nil)
}

// ExecuteWithPolicy runs the command with a specific policy.
func (r *BwrapRunner) ExecuteWithPolicy(ctx context.Context, command string, policy *Policy) (Result, error) {
	if err := r.validator.Validate(ctx, command); err != nil {
		return Result{}, fmt.Errorf("command validation failed: %w", err)
	}

	bwrapArgs := r.GetBwrapArgs(command, policy)
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

// ExecuteStream runs the command with a specific policy, streaming standard I/O.
func (r *BwrapRunner) ExecuteStream(ctx context.Context, command string, policy *Policy, stdin io.Reader, stdout, stderr io.Writer) (Result, error) {
	if err := r.validator.Validate(ctx, command); err != nil {
		return Result{}, fmt.Errorf("command validation failed: %w", err)
	}

	bwrapArgs := r.GetBwrapArgs(command, policy)
	cmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)

	var outBuf, errBuf bytes.Buffer
	if stdout != nil {
		cmd.Stdout = io.MultiWriter(stdout, &outBuf)
	} else {
		cmd.Stdout = &outBuf
	}

	if stderr != nil {
		cmd.Stderr = io.MultiWriter(stderr, &errBuf)
	} else {
		cmd.Stderr = &errBuf
	}

	if stdin != nil {
		cmd.Stdin = stdin
	}

	err := cmd.Run()
	exitCode := 0
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			exitCode = exitError.ExitCode()
            if exitCode == 1 && bytes.Contains(errBuf.Bytes(), []byte("bwrap:")) {
                violationCount.Add(ctx, 1)
            }
		} else {
			return Result{}, fmt.Errorf("failed to run bwrap: %w", err)
		}
	}

	return Result{
		Stdout:   outBuf.String(),
		Stderr:   errBuf.String(),
		ExitCode: exitCode,
	}, nil
}
