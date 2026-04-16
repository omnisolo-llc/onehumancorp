package harness

import (
	"context"
	"strings"
	"testing"
)

func TestHarness_Run(t *testing.T) {
	config := &SandboxConfig{
		AllowedDomains: []string{"example.com"},
		DeniedDomains:  []string{"malicious.com"},
		ReadPaths:      []string{"/usr", "/lib", "/lib64", "/bin", "/etc"},
		WritePaths:     []string{"/tmp"},
		EnableSeccomp:  false,
	}

	harness := NewHarness(config)

	// Test a simple command
	stdout, stderr, err := harness.Run(context.Background(), "/bin/echo", "hello bwrap")
	if err != nil {
		t.Fatalf("Expected no error, got %v\nstderr: %s", err, stderr)
	}

	if !strings.Contains(stdout, "hello bwrap") {
		t.Errorf("Expected 'hello bwrap' in stdout, got %q", stdout)
	}
}
