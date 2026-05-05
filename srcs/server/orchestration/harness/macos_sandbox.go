package harness

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os/exec"
	"regexp"
	"strings"
)

// SandboxAdapter interface matches the expected operations for a sandboxed execution wrapper.
type SandboxAdapter interface {
	WrapCommand(cmd string) (string, error)
	UpdateConfig(policyJSON string) error
	AnnotateError(err error, stdout string) string
}

// SandboxPolicy defines access constraints
type SandboxPolicy struct {
	DisabledCommands []string `json:"disabled_commands"`
	DisabledPatterns []string `json:"disabled_patterns"`
	ReadOnlyPaths    []string `json:"read_only_paths"`
	BlockedDomains   []string `json:"blocked_domains"`
}

// MacOSSandboxManager wraps command execution using sandbox-exec
type MacOSSandboxManager struct {
	policy SandboxPolicy
}

// NewMacOSSandboxManager creates a new MacOSSandboxManager
func NewMacOSSandboxManager() *MacOSSandboxManager {
	return &MacOSSandboxManager{
		policy: SandboxPolicy{},
	}
}

// WrapCommand implements SandboxAdapter.WrapCommand
func (m *MacOSSandboxManager) WrapCommand(cmd string) (string, error) {
	if !m.evaluate(cmd) {
		return "", fmt.Errorf("Command execution denied by sandbox policy")
	}

	profile := "(version 1)\n(allow default)\n"

	for _, path := range m.policy.ReadOnlyPaths {
		// Prevent profile injection by removing double quotes, backslashes, and single quotes in path
		safePath := strings.ReplaceAll(path, "\"", "")
		safePath = strings.ReplaceAll(safePath, "'", "")
		safePath = strings.ReplaceAll(safePath, "\\", "")
		profile += fmt.Sprintf("(deny file-write* (subpath \"%s\"))\n", safePath)
	}

	for _, domain := range m.policy.BlockedDomains {
		// Ensure domain is safe
		safeDomain := strings.ReplaceAll(domain, "\"", "")
		safeDomain = strings.ReplaceAll(safeDomain, "'", "")
		safeDomain = strings.ReplaceAll(safeDomain, "\\", "")
		// macOS sandbox requires port numbers, assuming outbound http/https or block all
		profile += fmt.Sprintf("(deny network-outbound (remote tcp \"%s:*\"))\n", safeDomain)
	}

	// Safely quote the command for bash.
	safeCmd := "'" + strings.ReplaceAll(cmd, "'", "'\\''") + "'"

	return fmt.Sprintf("sandbox-exec -p '%s' bash -c %s", profile, safeCmd), nil
}

// UpdateConfig implements SandboxAdapter.UpdateConfig
func (m *MacOSSandboxManager) UpdateConfig(policyJSON string) error {
	var policy SandboxPolicy
	err := json.Unmarshal([]byte(policyJSON), &policy)
	if err != nil {
		return fmt.Errorf("Invalid policy JSON: %v", err)
	}
	m.policy = policy
	return nil
}

// AnnotateError implements SandboxAdapter.AnnotateError
func (m *MacOSSandboxManager) AnnotateError(err error, stdout string) string {
	errStr := "nil"
	if err != nil {
		errStr = err.Error()
	}
	return fmt.Sprintf("SANDBOX_FAILURE: %v\nSTDOUT:\n%s", errStr, stdout)
}

func (m *MacOSSandboxManager) evaluate(cmd string) bool {
	// A more robust evaluation using word boundaries
	for _, disabled := range m.policy.DisabledCommands {
		// Create a regex to match the command with word boundaries to avoid sub-string matches
		// and detect commands even if prefixed by spaces or semicolons
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
func (m *MacOSSandboxManager) Execute(cmd string) (string, error) {
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
