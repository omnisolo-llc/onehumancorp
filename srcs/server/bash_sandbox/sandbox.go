package bash_sandbox

import (
	"context"

	"fmt"
	"os/exec"
	"regexp"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter         = otel.Meter("ohc_bash_sandbox")
	execCount     metric.Int64Counter
	violationCount metric.Int64Counter
	errorCount    metric.Int64Counter
)

func init() {
	var err error
	execCount, err = meter.Int64Counter("ohc_sandbox_exec_total",
		metric.WithDescription("Total number of bash sandbox execution attempts"))
	if err != nil {
		panic(err)
	}

	violationCount, err = meter.Int64Counter("ohc_sandbox_violation_total",
		metric.WithDescription("Total number of bash sandbox security violations"))
	if err != nil {
		panic(err)
	}

	errorCount, err = meter.Int64Counter("ohc_sandbox_error_total",
		metric.WithDescription("Total number of bash sandbox execution errors"))
	if err != nil {
		panic(err)
	}
}

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
			violationCount.Add(ctx, 1, metric.WithAttributes(attribute.String("pattern", pattern.String())))
			return fmt.Errorf("command violates security policy: matched %s", pattern.String())
		}
	}
	return nil
}

// ExecuteContext runs the command if it passes validation.
func (s *Sandbox) ExecuteContext(ctx context.Context, command string, workDir string) (string, error) {
	execCount.Add(ctx, 1)

	if err := s.ValidateContext(ctx, command); err != nil {
		return "", err
	}

	cmd := exec.CommandContext(ctx, "bash", "-c", command)
	if workDir != "" {
		cmd.Dir = workDir
	}
	out, err := cmd.CombinedOutput()
	if err != nil {
		errorCount.Add(ctx, 1)
		return string(out), fmt.Errorf("execution failed: %w", err)
	}

	return string(out), nil
}
