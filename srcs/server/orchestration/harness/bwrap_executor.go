package harness

import (
	"bytes"
	"context"
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
	"time"

	"onehumancorp/srcs/server/telemetry"
)

import "sync"

// BwrapSandboxManager wraps command execution using bubblewrap (bwrap)
type BwrapSandboxManager struct {
	policy     SandboxPolicy
	proxy      *ProxyServer
	socketPath string
	mu         sync.Mutex
}

// NewBwrapSandboxManager creates a new BwrapSandboxManager
func NewBwrapSandboxManager() *BwrapSandboxManager {
	return &BwrapSandboxManager{
		policy: SandboxPolicy{},
	}
}

// WrapCommand implements SandboxAdapter.WrapCommand
func (m *BwrapSandboxManager) WrapCommand(cmd string) (string, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

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
	// We MUST place --tmpfs BEFORE --bind
	bwrapArgs := []string{
		"--unshare-all",
		"--unshare-net", // explicitly document lack of network
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--tmpfs", "/tmp",
	}

	bashCmd := cmd

	if m.socketPath != "" {
		// Bind the socket path after /tmp is mounted as tmpfs
		bwrapArgs = append(bwrapArgs, "--bind", m.socketPath, m.socketPath)

		// Instead of HTTP_PROXY=unix://, we use socat inside the container to bridge a local TCP port to the Unix socket
		// We bring 'lo' up, then start socat in the background, and execute the original command
		bashCmd = fmt.Sprintf("ip link set lo up && socat TCP4-LISTEN:3128,fork,bind=127.0.0.1 UNIX-CONNECT:%s & HTTP_PROXY=http://127.0.0.1:3128 HTTPS_PROXY=http://127.0.0.1:3128 %s", m.socketPath, cmd)
	}

	// Safely quote the command for bash.
	safeCmd := "'" + strings.ReplaceAll(bashCmd, "'", "'\\''") + "'"

	// Construct final command
	fullCmd := fmt.Sprintf("bwrap %s -- bash -c %s", strings.Join(bwrapArgs, " "), safeCmd)

	return fullCmd, nil
}

// Close cleanly shuts down the proxy server if it is running
func (m *BwrapSandboxManager) Close() error {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.proxy != nil {
		err := m.proxy.Close()
		m.proxy = nil
		m.socketPath = ""
		return err
	}
	return nil
}

// UpdateConfig implements SandboxAdapter.UpdateConfig
func (m *BwrapSandboxManager) UpdateConfig(policyJSON string) error {
	var policy SandboxPolicy
	err := json.Unmarshal([]byte(policyJSON), &policy)
	if err != nil {
		return fmt.Errorf("Invalid policy JSON: %v", err)
	}

	m.mu.Lock()
	defer m.mu.Unlock()

	m.policy = policy

	// Restart proxy with new policy
	if m.proxy != nil {
		m.proxy.Close()
		m.proxy = nil
	}

	b := make([]byte, 8)
	rand.Read(b)
	m.socketPath = filepath.Join("/tmp", fmt.Sprintf("harness-proxy-%s.sock", hex.EncodeToString(b)))

	server, err := StartProxy(m.policy, m.socketPath)
	if err != nil {
		return fmt.Errorf("failed to start proxy: %w", err)
	}

	m.proxy = server
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
