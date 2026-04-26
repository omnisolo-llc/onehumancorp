package bash_sandbox

import (
	"context"

	"fmt"
	"os"
	"os/exec"
	"regexp"
	"strings"
	"sync"
	"time"
	"runtime"

	"github.com/onehumancorp/mono/src/server/harness/validation"
	"github.com/onehumancorp/mono/src/server/telemetry"
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

type SandboxViolation struct {
	Command string
	Error   string
	Time    time.Time
}

type SandboxViolationStore struct {
	mu         sync.Mutex
	violations []SandboxViolation
}

func NewSandboxViolationStore() *SandboxViolationStore {
	return &SandboxViolationStore{
		violations: make([]SandboxViolation, 0),
	}
}

func (s *SandboxViolationStore) RecordViolation(ctx context.Context, cmd string, errDetails string) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.violations = append(s.violations, SandboxViolation{
		Command: cmd,
		Error:   errDetails,
		Time:    time.Now(),
	})
	violationCount.Add(ctx, 1, metric.WithAttributes(attribute.String("error", errDetails)))
}

func (s *SandboxViolationStore) GetViolations() []SandboxViolation {
	s.mu.Lock()
	defer s.mu.Unlock()
	res := make([]SandboxViolation, len(s.violations))
	copy(res, s.violations)
	return res
}

// ExecutionEnvironment defines the interface for command execution.
type ExecutionEnvironment interface {
	ExecuteContext(ctx context.Context, command string, workDir string) (string, error)
}

// Sandbox defines the configuration for secure bash execution.
type LocalEnvironment struct {
	blockedPatterns []*regexp.Regexp
	violationStore  *SandboxViolationStore
}

// NewSandbox creates a new Sandbox with default security rules.
func NewSandbox() ExecutionEnvironment {
	return &LocalEnvironment{
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
		violationStore: NewSandboxViolationStore(),
	}
}

func wrapCommandWithSandboxMacOS(ctx context.Context, command string, workDir string) (*exec.Cmd, func(), error) {
	// Create a temporary sandbox profile file
	profileContent := `
(version 1)
(deny default)
(allow file-read*)
(allow process-exec)
(allow file-write* (subpath "/tmp"))
(allow file-write* (subpath (param "WORKDIR")))
(allow network*)
(allow sysctl-read)
`
	tmpFile, err := os.CreateTemp("", "sandbox_profile_*.sb")
	if err != nil {
	    // fail closed
	    return nil, func() {}, fmt.Errorf("failed to create sandbox profile: %w", err)
	}
	tmpFile.Write([]byte(profileContent))
	tmpFile.Close()

	cleanup := func() {
	    os.Remove(tmpFile.Name())
	}

	cmd := exec.CommandContext(ctx, "sandbox-exec", "-f", tmpFile.Name(), "-D", "WORKDIR="+workDir, "bash", "-c", command)
	return cmd, cleanup, nil
}

// ValidateContext checks if the command violates any security rules with context.
func (s *LocalEnvironment) ValidateContext(ctx context.Context, command string) error {
	validator := validation.NewBashASTValidator(s.violationStore)
	if err := validator.Validate(ctx, command); err != nil {
		telemetry.RecordBubblewrapViolation(ctx)
		return fmt.Errorf("command violates security policy: %v", err)
	}
	for _, pattern := range s.blockedPatterns {
		if pattern.MatchString(command) {
			s.violationStore.RecordViolation(ctx, command, "matched " + pattern.String())
			telemetry.RecordBubblewrapViolation(ctx)
			return fmt.Errorf("command violates security policy: matched %s", pattern.String())
		}
	}
	return nil
}

// ExecuteContext runs the command if it passes validation.
func (s *LocalEnvironment) ExecuteContext(ctx context.Context, command string, workDir string) (string, error) {
	execCount.Add(ctx, 1)
	telemetry.RecordBubblewrapSpawn(ctx)

	startTime := time.Now()
	defer func() {
		telemetry.RecordBubblewrapExecutionLatency(ctx, time.Since(startTime).Seconds())
	}()

	if err := s.ValidateContext(ctx, command); err != nil {
		return fmt.Sprintf("<sandbox_violations>%v</sandbox_violations>", err), err
	}

	var cmd *exec.Cmd

	homeDir := workDir
	if homeDir == "" {
		homeDir = os.TempDir()
	}
    workDirToUse := workDir
    if workDirToUse == "" {
        workDirToUse = homeDir
    }

    var cleanup func()
    cleanup = func() {}

	if runtime.GOOS == "darwin" {
	    var err error
	    _, lookErr := exec.LookPath("sandbox-exec")
		if lookErr != nil {
			cmd = exec.CommandContext(ctx, "bash", "-c", command)
		} else {
		    cmd, cleanup, err = wrapCommandWithSandboxMacOS(ctx, command, workDirToUse)
		    if err != nil {
		        return "", err
		    }
		    defer cleanup()
		}
	} else if runtime.GOOS == "linux" {
	    // Check if bwrap exists. If not, fallback to bash
		_, err := exec.LookPath("bwrap")
		if err != nil {
			cmd = exec.CommandContext(ctx, "bash", "-c", command)
		} else {
		    var err error
		    cmd, cleanup, err = wrapCommandWithSandboxLinux(ctx, command, workDirToUse)
		    if err != nil {
		        return "", err
		    }
		    defer cleanup()
        }
	} else {
		cmd = exec.CommandContext(ctx, "bash", "-c", command)
	}

	cmd.Env = []string{} // Clear inherited environment explicitly to prevent secret leaks
	// Pass through environment variables except sensitive ones
	blockedEnvPrefixes := []string{"OHC_API_KEY=", "GH_TOKEN=", "GITHUB_TOKEN=", "OTEL_EXPORTER_OTLP_HEADERS="}
	for _, env := range os.Environ() {
		blocked := false
		for _, prefix := range blockedEnvPrefixes {
			if strings.HasPrefix(env, prefix) {
				blocked = true
				break
			}
		}
		if !blocked && !strings.HasPrefix(env, "HOME=") {
			cmd.Env = append(cmd.Env, env)
		}
	}
	cmd.Env = append(cmd.Env, "HOME="+homeDir)
	if workDir != "" {
		cmd.Dir = workDir
	}
	out, err := cmd.CombinedOutput()
	if err != nil {
		errorCount.Add(ctx, 1)

		outputStr := string(out)
		if strings.Contains(outputStr, "Operation not permitted") {
			s.violationStore.RecordViolation(ctx, command, "Operation not permitted")
			outputStr += "\n<sandbox_violations>Operation not permitted: sandbox boundary drop</sandbox_violations>"
			telemetry.RecordBubblewrapViolation(ctx)
		} else if strings.Contains(outputStr, "Permission denied") {
			s.violationStore.RecordViolation(ctx, command, "Permission denied")
			outputStr += "\n<sandbox_violations>Permission denied: sandbox boundary drop</sandbox_violations>"
			telemetry.RecordBubblewrapViolation(ctx)
		}

		return outputStr, fmt.Errorf("execution failed: %w", err)
	}

	return string(out), nil
}
