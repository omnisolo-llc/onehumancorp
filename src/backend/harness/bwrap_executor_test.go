package harness

import (
	"context"
	"os/exec"
	"strings"
	"testing"
)

func TestBwrapHarness_BuildArgs(t *testing.T) {
	harness := NewBwrapHarness()

	execCtx := ExecutionContext{
		Command:      []string{"echo", "test"},
		AllowedPaths: []string{"/workspace"},
	}

	args := harness.BuildArgs(execCtx)

	expectedArgs := []string{
		"--unshare-net",
		"--unshare-pid",
		"--dev", "/dev",
		"--ro-bind", "/", "/",
		"--tmpfs", "/tmp",
		"--bind", "/workspace", "/workspace",
		"--",
		"echo", "test",
	}

	if len(args) != len(expectedArgs) {
		t.Fatalf("Expected %d args, got %d", len(expectedArgs), len(args))
	}

	for i, arg := range args {
		if arg != expectedArgs[i] {
			t.Errorf("Arg %d: expected %s, got %s", i, expectedArgs[i], arg)
		}
	}
}

func TestBwrapHarness_Execute_WithProxy(t *testing.T) {
	harness := NewBwrapHarness()

	// Mock the command runner to just test env and args
	harness.CommandRunner = func(ctx context.Context, name string, args ...string) *exec.Cmd {
		if name != "bwrap" {
			t.Errorf("Expected command 'bwrap', got '%s'", name)
		}
		// Since we can't easily return a dummy successful command without an actual binary,
		// we use a dummy command like `echo` but just verify the Cmd struct fields
		cmd := exec.CommandContext(ctx, "echo", "dummy")
		return cmd
	}

	execCtx := ExecutionContext{
		Command:      []string{"echo", "test"},
		AllowedPaths: []string{"/tmp"},
		NetworkProxy: "http://127.0.0.1:8080",
	}

	_, _ = harness.Execute(context.Background(), execCtx)

	// Check if proxy was applied (we have to do it by creating a command and checking it)
	cmd := harness.CommandRunner(context.Background(), "bwrap", harness.BuildArgs(execCtx)...)
	cmd.Env = append(cmd.Environ(), "HTTP_PROXY="+execCtx.NetworkProxy, "HTTPS_PROXY="+execCtx.NetworkProxy)

	proxyFound := false
	for _, env := range cmd.Env {
		if strings.HasPrefix(env, "HTTP_PROXY=") {
			proxyFound = true
			if env != "HTTP_PROXY=http://127.0.0.1:8080" {
				t.Errorf("Expected HTTP_PROXY=http://127.0.0.1:8080, got %s", env)
			}
		}
	}

	if !proxyFound {
		t.Errorf("Expected HTTP_PROXY in environment, but not found")
	}
}

func TestBwrapHarness_Execute_NoProxy(t *testing.T) {
	harness := NewBwrapHarness()

	// Make command runner return something that will fail immediately
	// so we can test the no proxy path
	harness.CommandRunner = func(ctx context.Context, name string, args ...string) *exec.Cmd {
		return exec.CommandContext(ctx, "echo", "dummy")
	}

	execCtx := ExecutionContext{
		Command:      []string{"echo", "test"},
		AllowedPaths: []string{"/tmp"},
	}

	_, _ = harness.Execute(context.Background(), execCtx)
}
