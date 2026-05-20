package sandbox

import (
	"os/exec"
	"strings"
	"testing"
)

func TestBashWrapperDefault(t *testing.T) {
	bw := NewBashWrapper()
	wrapped := bw.Wrap("echo hello")

	bwrapPath, err := exec.LookPath("bwrap")
	hasBwrap := err == nil

	var expected string
	if hasBwrap {
		expected = bwrapPath + " --unshare-all --share-net --ro-bind / / --tmpfs /tmp --tmpfs /var -- bash -c 'set -e; umask 077; echo hello'"
	} else {
		expected = "bash -c 'set -e; umask 077; echo hello'"
	}

	if wrapped != expected {
		t.Errorf("Expected %q, got %q", expected, wrapped)
	}
}

func TestBashWrapperWithPolicy(t *testing.T) {
	bw := NewBashWrapper()
	policy := SandboxPolicy{
		ReadOnlyPaths:  []string{"/etc", "/var"},
		BlockedDomains: []string{"evil.com"},
	}
	bw.UpdatePolicy(policy)

	wrapped := bw.Wrap("echo hello")

	_, err := exec.LookPath("bwrap")
	hasBwrap := err == nil

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
	if !strings.Contains(wrapped, "echo hello") {
		t.Errorf("Missing original command in wrapped command: %s", wrapped)
	}
}
