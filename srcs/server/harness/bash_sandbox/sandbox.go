package bash_sandbox

import (
	"bytes"
	"context"
	"errors"
	"os/exec"
	"regexp"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

var (
	// Dangerous patterns
	zshExpansionRegex   = regexp.MustCompile(`\$\{?[\w~]+[#%^=]`)
	processSubRegex     = regexp.MustCompile(`[><]\(`)
	legacyExpansionRegex = regexp.MustCompile(`\$\[`)
	zmodloadRegex       = regexp.MustCompile(`\bzmodload\b`)
)

type SandboxPolicy struct {
	AllowedHosts     []string
	ReadRestriction  []string
	WriteRestriction []string
}

type Result struct {
	Stdout   string
	Stderr   string
	ExitCode int
	Error    error
}

func ValidateBashCommand(cmd string) error {
	if zshExpansionRegex.MatchString(cmd) {
		return errors.New("zsh expansion detected")
	}
	if processSubRegex.MatchString(cmd) {
		return errors.New("process substitution detected")
	}
	if legacyExpansionRegex.MatchString(cmd) {
		return errors.New("legacy expansion detected")
	}
	if zmodloadRegex.MatchString(cmd) {
		return errors.New("zmodload detected")
	}
	return nil
}

func RunSandboxed(ctx context.Context, cmdStr string, policy SandboxPolicy) (Result, error) {
	start := time.Now()
	defer func() {
		telemetry.RecordBashExecutionDuration(ctx, time.Since(start).Seconds())
	}()

	if err := ValidateBashCommand(cmdStr); err != nil {
		telemetry.RecordBashSandboxViolation(ctx)
		return Result{}, err
	}

	cmd := exec.CommandContext(ctx, "bash", "-c", cmdStr)

	// Basic directory isolation
	if len(policy.WriteRestriction) > 0 {
		cmd.Dir = policy.WriteRestriction[0]
	}

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()
	var exitCode int
	if err != nil {
		if exitError, ok := err.(*exec.ExitError); ok {
			exitCode = exitError.ExitCode()
		} else {
			exitCode = -1
		}
	} else {
		exitCode = cmd.ProcessState.ExitCode()
	}

	return Result{
		Stdout:   stdout.String(),
		Stderr:   stderr.String(),
		ExitCode: exitCode,
		Error:    err,
	}, nil
}
