package sandbox

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"regexp"
	"strings"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter          = otel.Meter("ohc_agent_sandbox")
	execCount      metric.Int64Counter
	violationCount metric.Int64Counter
	errorCount     metric.Int64Counter
)

func init() {
	var err error
	execCount, err = meter.Int64Counter("ohc_sandbox_exec_total",
		metric.WithDescription("Total number of bash sandbox execution attempts"))
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to initialize metric: %v\n", err)
	}

	violationCount, err = meter.Int64Counter("ohc_sandbox_violations_total",
		metric.WithDescription("Total number of bash sandbox security violations"))
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to initialize metric: %v\n", err)
	}

	errorCount, err = meter.Int64Counter("ohc_sandbox_error_total",
		metric.WithDescription("Total number of bash sandbox execution errors"))
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to initialize metric: %v\n", err)
	}
}

type SandboxManager struct {
	SandboxDir      string
	blockedPatterns []*regexp.Regexp
}

func NewSandboxManager() (*SandboxManager, error) {
	dir, err := os.MkdirTemp("", "ohc_sandbox_*")
	if err != nil {
		return nil, err
	}
	if err := os.Chmod(dir, 0700); err != nil {
		os.RemoveAll(dir)
		return nil, err
	}
	return &SandboxManager{
		SandboxDir: dir,
		blockedPatterns: []*regexp.Regexp{
			regexp.MustCompile(`(?i)\bsudo\b`),
			regexp.MustCompile(`(?i)\brm\s+-rf\s+/`),
			regexp.MustCompile(`(?i)\bchown\b`),
			regexp.MustCompile(`(?i)\bchmod\b`),
			regexp.MustCompile(`<\(`),
			regexp.MustCompile(`>\(`),
			regexp.MustCompile(`=\(`),
		},
	}, nil
}

func (s *SandboxManager) ValidateContext(ctx context.Context, command string) error {
	for _, pattern := range s.blockedPatterns {
		if pattern.MatchString(command) {
			if violationCount != nil {
				violationCount.Add(ctx, 1, metric.WithAttributes(attribute.String("pattern", pattern.String())))
			}
			return fmt.Errorf("command violates security policy: matched %s", pattern.String())
		}
	}
	return nil
}

func (s *SandboxManager) Execute(ctx context.Context, cmdStr string, workDir string) (string, error) {
	if execCount != nil {
		execCount.Add(ctx, 1)
	}

	if err := s.ValidateContext(ctx, cmdStr); err != nil {
		return fmt.Sprintf("<sandbox_violations>%v</sandbox_violations>", err), err
	}

	wrapperCmd := fmt.Sprintf("shopt -u extglob 2>/dev/null || true; %s", cmdStr)
	cmd := exec.CommandContext(ctx, "bash", "-c", wrapperCmd)
	if workDir != "" {
		cmd.Dir = workDir
	} else {
		cmd.Dir = s.SandboxDir
	}

	cmd.Env = os.Environ() // keep environment as before so the agent can interact
	// ensure TMPDIR is isolated
	foundTmpDir := false
	for i, env := range cmd.Env {
		if len(env) > 7 && env[:7] == "TMPDIR=" {
			cmd.Env[i] = fmt.Sprintf("TMPDIR=%s", s.SandboxDir)
			foundTmpDir = true
			break
		}
	}
	if !foundTmpDir {
		cmd.Env = append(cmd.Env, fmt.Sprintf("TMPDIR=%s", s.SandboxDir))
	}

	out, err := cmd.CombinedOutput()
	if err != nil {
		if errorCount != nil {
			errorCount.Add(ctx, 1)
		}

		outputStr := string(out)
		if strings.Contains(outputStr, "Operation not permitted") {
			outputStr += "\n<sandbox_violations>Operation not permitted: sandbox boundary drop</sandbox_violations>"
		} else if strings.Contains(outputStr, "Permission denied") {
			outputStr += "\n<sandbox_violations>Permission denied: sandbox boundary drop</sandbox_violations>"
		}

		return outputStr, fmt.Errorf("execution failed: %w", err)
	}

	return string(out), nil
}

func (s *SandboxManager) Cleanup() error {
	return os.RemoveAll(s.SandboxDir)
}
