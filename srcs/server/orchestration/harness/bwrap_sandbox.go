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

// BwrapSandboxManager wraps command execution using bubblewrap (bwrap)
type BwrapSandboxManager struct {
	policy SandboxPolicy
}

// NewBwrapSandboxManager creates a new BwrapSandboxManager
func NewBwrapSandboxManager() *BwrapSandboxManager {
	return &BwrapSandboxManager{
		policy: SandboxPolicy{},
	}
}

// WrapCommand implements SandboxAdapter.WrapCommand
func (m *BwrapSandboxManager) WrapCommand(cmd string) (string, error) {
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

	// Construct final command
	fullCmd := fmt.Sprintf("bwrap %s -- bash -c %s", strings.Join(bwrapArgs, " "), safeCmd)

	return fullCmd, nil
}

// UpdateConfig implements SandboxAdapter.UpdateConfig
func (m *BwrapSandboxManager) UpdateConfig(policyJSON string) error {
	var policy SandboxPolicy
	err := json.Unmarshal([]byte(policyJSON), &policy)
	if err != nil {
		return fmt.Errorf("Invalid policy JSON: %v", err)
	}
	m.policy = policy
	return nil
}

// AnnotateError implements SandboxAdapter.AnnotateError
func (m *BwrapSandboxManager) AnnotateError(err error, stdout string) string {
	errStr := "nil"
	if err != nil {
		errStr = err.Error()
	}
	return fmt.Sprintf("BWRAP_FAILURE: %v\nSTDOUT:\n%s", errStr, stdout)
}

func (m *BwrapSandboxManager) evaluate(cmd string) bool {
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
func (m *BwrapSandboxManager) Execute(cmd string) (string, error) {
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
