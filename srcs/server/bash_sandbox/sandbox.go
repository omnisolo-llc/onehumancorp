package bash_sandbox

import (
	"context"

	"fmt"
	"os/exec"
	"regexp"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"time"
)

// Sandbox defines the configuration for secure bash execution.
type Sandbox struct {
	blockedPatterns []*regexp.Regexp
}

// NewSandbox creates a new Sandbox with default security rules.
func NewSandbox() *Sandbox {
	return &Sandbox{
		blockedPatterns: []*regexp.Regexp{
			regexp.MustCompile(`(?i)\bsudo\b`),
			regexp.MustCompile(`(?i)\brm\s+-rf\s+/`),
			regexp.MustCompile(`(?i)\bchown\b`),
			regexp.MustCompile(`(?i)\bchmod\b`),
			// Prevent some zsh specific or process substitutions if needed
			regexp.MustCompile(`<\(`),
			regexp.MustCompile(`>\(`),
			regexp.MustCompile(`=\(`),
		},
	}
}

// ValidateContext checks if the command violates any security rules with context.
func (s *Sandbox) ValidateContext(ctx context.Context, command string) error {
	for _, pattern := range s.blockedPatterns {
		if pattern.MatchString(command) {
			telemetry.RecordBubblewrapViolation(ctx, pattern.String())
			return fmt.Errorf("command violates security policy: matched %s", pattern.String())
		}
	}
	return nil
}

// ExecuteContext runs the command if it passes validation.
func (s *Sandbox) ExecuteContext(ctx context.Context, command string, workDir string) (string, error) {
	telemetry.RecordBubblewrapSpawn(ctx)
	start := time.Now()

	if err := s.ValidateContext(ctx, command); err != nil {
		return "", err
	}

	cmd := exec.CommandContext(ctx, "bash", "-c", command)
	if workDir != "" {
		cmd.Dir = workDir
	}
	out, err := cmd.CombinedOutput()
	if err != nil {

		telemetry.RecordBubblewrapError(ctx)
		return string(out), fmt.Errorf("execution failed: %w", err)
	}

	telemetry.RecordBubblewrapExecutionLatency(ctx, time.Since(start).Seconds())
	return string(out), nil
}
