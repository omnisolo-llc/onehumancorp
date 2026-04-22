package harness

import (
	"bytes"
	"context"
	"fmt"
	"os/exec"
)

// MacOsSandboxRunner executes commands inside a macOS sandbox-exec sandbox.
type MacOsSandboxRunner struct {
	validator *ASTValidator
}

// NewMacOsSandboxRunner creates a new MacOsSandboxRunner.
func NewMacOsSandboxRunner(validator *ASTValidator) *MacOsSandboxRunner {
	if validator == nil {
		validator = NewASTValidator()
	}
	return &MacOsSandboxRunner{
		validator: validator,
	}
}

// GenerateProfile generates the sandbox-exec profile for a given policy.
func (r *MacOsSandboxRunner) GenerateProfile(policy *Policy) string {
	var profile bytes.Buffer
	profile.WriteString("(version 1)\n")

	// Default to denying everything, then open up permissions
	profile.WriteString("(deny default)\n")
	profile.WriteString("(allow process-exec)\n")
	profile.WriteString("(allow process-fork)\n")

	if policy == nil || !policy.AllowNetwork {
		profile.WriteString("(deny network*)\n")
	} else {
		profile.WriteString("(allow network*)\n")
	}

	// Default read-only file access similar to bwrap root bind
	profile.WriteString("(allow file-read* (subpath \"/\"))\n")

	if policy != nil {
		for _, path := range policy.AllowedPaths {
			profile.WriteString(fmt.Sprintf("(allow file-write* (subpath %q))\n", path))
		}
		// ReadOnlyPaths are already covered by the global file-read* above,
		// but we can be explicit or just let the global handle it.
		// For BlockedPaths, we explicitly deny them
		for _, path := range policy.BlockedPaths {
			profile.WriteString(fmt.Sprintf("(deny file* (subpath %q))\n", path))
		}
	}

	return profile.String()
}

// GetSandboxExecArgs generates the sandbox-exec arguments for a given command.
func (r *MacOsSandboxRunner) GetSandboxExecArgs(command string, policy *Policy) []string {
	profile := r.GenerateProfile(policy)
	return []string{"-p", profile, "bash", "-c", command}
}

// Execute runs the command in a sandbox-exec sandbox after AST validation.
func (r *MacOsSandboxRunner) Execute(ctx context.Context, command string) (Result, error) {
	return r.ExecuteWithPolicy(ctx, command, nil)
}

// ExecuteWithPolicy runs the command with a specific policy.
func (r *MacOsSandboxRunner) ExecuteWithPolicy(ctx context.Context, command string, policy *Policy) (Result, error) {
	if err := r.validator.Validate(ctx, command); err != nil {
		return Result{}, fmt.Errorf("command validation failed: %w", err)
	}

	args := r.GetSandboxExecArgs(command, policy)
	cmd := exec.CommandContext(ctx, "sandbox-exec", args...)

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()
	exitCode := 0
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			exitCode = exitError.ExitCode()
			// sandbox-exec uses exit code 71 (EX_OSERR) for profile syntax errors.
			// We track profile compilation failures as sandbox violations for metrics.
			if exitCode == 71 {
				violationCount.Add(ctx, 1)
			}
		} else {
			return Result{}, fmt.Errorf("failed to run sandbox-exec: %w", err)
		}
	}

	return Result{
		Stdout:   stdout.String(),
		Stderr:   stderr.String(),
		ExitCode: exitCode,
	}, nil
}
