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

	"onehumancorp/srcs/server/harness/network"
	"onehumancorp/srcs/server/telemetry"
)

// BwrapSandboxManager wraps command execution using bubblewrap (bwrap)
type BwrapSandboxManager struct {
	policy      SandboxPolicy
	proxyBridge *network.NetworkBridgeProxy
}

// NewBwrapSandboxManager creates a new BwrapSandboxManager
func NewBwrapSandboxManager() *BwrapSandboxManager {
	proxyBridge := network.NewNetworkBridgeProxy("/tmp/ohc-agent-http.sock", []string{})
	proxyBridge.Start()

	return &BwrapSandboxManager{
		policy:      SandboxPolicy{},
		proxyBridge: proxyBridge,
	}
}

// WrapCommand implements SandboxAdapter.WrapCommand
func (m *BwrapSandboxManager) WrapCommand(cmd string) (string, error) {
	if !m.evaluate(cmd) {
		telemetry.RecordBubblewrapViolation(context.Background(), "policy_denied")
		telemetry.RecordHarnessViolation(context.Background(), "policy_denied")
		return "", fmt.Errorf("Command execution denied by sandbox policy")
	}

	// For metrics, try to extract the base command name (e.g. "ls" from "ls -la")
	cmdParts := strings.Fields(cmd)
	if len(cmdParts) > 0 {
		telemetry.RecordHarnessToolInvocation(context.Background(), cmdParts[0])
	}

	// Wait for the unix socket to be ready
	for i := 0; i < 10; i++ {
		if m.proxyBridge != nil && m.proxyBridge.IsReady() {
			break
		}
		time.Sleep(100 * time.Millisecond)
	}

	// Basic bwrap args: unshare all namespaces, bind root
	bwrapArgs := []string{
		"--unshare-all",
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--tmpfs", "/tmp",
	}

	if m.proxyBridge != nil {
		bwrapArgs = append(bwrapArgs, "--bind", m.proxyBridge.SocketPath, m.proxyBridge.SocketPath)
	}

	// Wrap the inner command to inject socat proxy and environment variables
	innerCmd := fmt.Sprintf(`socat TCP-LISTEN:8080,fork UNIX-CLIENT:%s & SOCAT_PID=$!; sleep 0.1; HTTP_PROXY=http://127.0.0.1:8080 HTTPS_PROXY=http://127.0.0.1:8080 ALL_PROXY=http://127.0.0.1:8080 %s; EXIT_CODE=$?; kill -9 $SOCAT_PID 2>/dev/null; exit $EXIT_CODE`, m.proxyBridge.SocketPath, cmd)

	// Safely quote the command for bash.
	safeCmd := "'" + strings.ReplaceAll(innerCmd, "'", "'\\''") + "'"

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
		telemetry.RecordBubblewrapExecutionLatency(context.Background(), durationSecs)
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
	telemetry.RecordBubblewrapSpawn(context.Background())
	err = c.Run()
	if err != nil {
		return stderr.String(), err
	}
	return out.String(), nil
}
