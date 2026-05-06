package harness

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
	"regexp"
	"strings"
	"time"

	"onehumancorp/srcs/server/telemetry"
)

// BwrapExecutor wraps command execution using bubblewrap (bwrap)
type BwrapExecutor struct {
	policy SandboxPolicy
}

// NewBwrapExecutor creates a new BwrapExecutor
func NewBwrapExecutor() *BwrapExecutor {
	return &BwrapExecutor{
		policy: SandboxPolicy{},
	}
}

// WrapCommand implements SandboxAdapter.WrapCommand
func (m *BwrapExecutor) WrapCommand(cmd string) (string, error) {
	if !m.evaluate(cmd) {
		telemetry.RecordHarnessViolation(context.Background(), "policy_denied")
		return "", fmt.Errorf("Command execution denied by sandbox policy")
	}

	// For metrics, try to extract the base command name (e.g. "ls" from "ls -la")
	cmdParts := strings.Fields(cmd)
	if len(cmdParts) > 0 {
		telemetry.RecordHarnessToolInvocation(context.Background(), cmdParts[0])
	}

	// Basic bwrap args: unshare all namespaces, bind root
	bwrapArgs := []string{
		"--unshare-all",
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--tmpfs", "/tmp",
	}

	// Safely quote the command for bash.
	safeCmd := "'" + strings.ReplaceAll(cmd, "'", "'\\''") + "'"

	// Construct final command. Inject HTTP_PROXY to route through our proxy.
	// We bind it inside the bash execution so sub-agents use the local proxy.
	proxyEnv := "HTTP_PROXY=http://127.0.0.1:8080 HTTPS_PROXY=http://127.0.0.1:8080"
	fullCmd := fmt.Sprintf("bwrap %s -- env %s bash -c %s", strings.Join(bwrapArgs, " "), proxyEnv, safeCmd)

	return fullCmd, nil
}

// UpdateConfig implements SandboxAdapter.UpdateConfig
func (m *BwrapExecutor) UpdateConfig(policyJSON string) error {
	var policy SandboxPolicy
	err := json.Unmarshal([]byte(policyJSON), &policy)
	if err != nil {
		return fmt.Errorf("Invalid policy JSON: %v", err)
	}
	m.policy = policy
	return nil
}

// AnnotateError implements SandboxAdapter.AnnotateError
func (m *BwrapExecutor) AnnotateError(err error, stdout string) string {
	errStr := "nil"
	if err != nil {
		errStr = err.Error()
	}
	return fmt.Sprintf("BWRAP_FAILURE: %v\nSTDOUT:\n%s", errStr, stdout)
}

func (m *BwrapExecutor) evaluate(cmd string) bool {
	// A more robust evaluation using word boundaries
	for _, disabled := range m.policy.DisabledCommands {
		re, err := regexp.Compile(`(?:^|[\s;&|])` + regexp.QuoteMeta(disabled) + `(?:[\s;&|]|$)`)
		if err == nil && re.MatchString(cmd) {
			return false
		}
	}
	for _, pattern := range m.policy.DisabledPatterns {
		if strings.Contains(cmd, pattern) {
			return false
		}
	}
	return true
}

// Execute is a helper that wraps the command and executes it locally for testing
func (m *BwrapExecutor) Execute(cmd string) (string, error) {
	start := time.Now()
	defer func() {
		durationSecs := time.Since(start).Seconds()
		telemetry.RecordHarnessExecutionDuration(context.Background(), durationSecs)
	}()

	wrapped, err := m.WrapCommand(cmd)
	if err != nil {
		return "", err
	}

	c := exec.Command("sh", "-c", wrapped)
	var out bytes.Buffer
	var stderr bytes.Buffer
	c.Stdout = &out
	c.Stderr = &stderr
	err = c.Run()
	if err != nil {
		return stderr.String(), err
	}
	return out.String(), nil
}
