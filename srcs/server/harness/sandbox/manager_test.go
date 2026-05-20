package sandbox

import (
	"context"
	"fmt"
	"os/exec"
	"strings"
	"testing"
)

func TestSandboxManagerAllowedCommand(t *testing.T) {
	sm := GetSandboxManager()
	ctx := context.Background()

	wrapped, err := sm.WrapCommand(ctx, "echo 'hello world'")
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	bwrapPath, err := exec.LookPath("bwrap")
	hasBwrap := err == nil

	var expected string
	if hasBwrap {
		expected = bwrapPath + " --unshare-all --share-net --ro-bind / / --tmpfs /tmp --tmpfs /var -- bash -c 'set -e; umask 077; echo '\\''hello world'\\'''"
	} else {
		expected = "bash -c 'set -e; umask 077; echo '\\''hello world'\\'''"
	}

	if wrapped != expected {
		t.Errorf("Expected %q, got %q", expected, wrapped)
	}
}

func TestSandboxManagerDeniedCommand(t *testing.T) {
	sm := GetSandboxManager()
	ctx := context.Background()

	_, err := sm.WrapCommand(ctx, "rm -rf /")
	if err == nil {
		t.Fatalf("Expected error, got nil")
	}
	if err.Error() != "Command execution denied by sandbox policy" {
		t.Errorf("Unexpected error message: %v", err)
	}
}

func TestSandboxManagerDeniedPattern(t *testing.T) {
	sm := GetSandboxManager()
	ctx := context.Background()

	_, err := sm.WrapCommand(ctx, "sudo ls")
	if err == nil {
		t.Fatalf("Expected error, got nil")
	}
	if err.Error() != "Command execution denied by sandbox policy" {
		t.Errorf("Unexpected error message: %v", err)
	}
}

func TestSandboxManagerUpdatePolicy(t *testing.T) {
	sm := GetSandboxManager()
	ctx := context.Background()

	policy := SandboxPolicy{
		DisabledCommands: []string{"curl"},
		DisabledPatterns: []string{`(?i)\bwget\b`},
		ReadOnlyPaths:    []string{"/etc", "/var"},
		BlockedDomains:   []string{"evil.com"},
	}
	sm.UpdatePolicy(policy)

	// test disabled command
	_, err := sm.WrapCommand(ctx, "curl http://example.com")
	if err == nil {
		t.Errorf("Expected curl to be denied")
	}

	// test disabled pattern
	_, err = sm.WrapCommand(ctx, "WGET http://example.com")
	if err == nil {
		t.Errorf("Expected wget to be denied")
	}

	// test wrap
	wrapped, err := sm.WrapCommand(ctx, "echo 'test'")
	if err != nil {
		t.Fatalf("Expected echo to be allowed, got error: %v", err)
	}

	bwrapPath, err := exec.LookPath("bwrap")
	hasBwrap := err == nil
	_ = bwrapPath

	if hasBwrap {
		if !strings.Contains(wrapped, "--ro-bind '/etc' '/etc'") {
			t.Errorf("Missing --ro-bind '/etc' '/etc' in wrapped command: %s", wrapped)
		}
		if !strings.Contains(wrapped, "--ro-bind '/var' '/var'") {
			t.Errorf("Missing --ro-bind '/var' '/var' in wrapped command: %s", wrapped)
		}
	}
	if !strings.Contains(wrapped, "export BLOCKED_DOMAINS='evil.com'") {
		t.Errorf("Missing BLOCKED_DOMAINS in wrapped command: %s", wrapped)
	}
}

func TestAnnotateError(t *testing.T) {
	sm := GetSandboxManager()
	err := fmt.Errorf("Command execution denied by sandbox policy")
	stdout := "some output"

	msg := sm.AnnotateError(err, stdout)
	expected := "SANDBOX_FAILURE: Command execution denied by sandbox policy\nSTDOUT:\nsome output"

	if msg != expected {
		t.Errorf("Expected %q, got %q", expected, msg)
	}
}
