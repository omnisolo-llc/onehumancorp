package bash_sandbox

import (
	"context"
	"os"
	"reflect"
	"testing"
)

func TestLinuxSandbox(t *testing.T) {
	workspaceDir, err := os.MkdirTemp("", "sandbox_workspace")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(workspaceDir)

	sandbox := NewLinuxSandbox("bwrap", workspaceDir)

	ctx := context.Background()
	output, err := sandbox.Run(ctx, "echo", []string{"hello world"})
	if err != nil {
		t.Skipf("bwrap is likely not installed, skipping test. err: %v", err)
	}

	if output != "hello world\n" {
		t.Errorf("expected 'hello world', got '%s'", output)
	}
}

func TestGetBwrapArgs(t *testing.T) {
	sandbox := NewLinuxSandbox("bwrap", "/workspace")
	args := sandbox.getBwrapArgs("echo", []string{"hello"})

	expectedArgs := []string{
		"--unshare-net",
		"--unshare-pid",
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--bind", "/workspace", "/workspace",
		"--die-with-parent",
		"--chdir", "/workspace",
		"echo",
		"hello",
	}

	if !reflect.DeepEqual(args, expectedArgs) {
		t.Errorf("getBwrapArgs returned %v, expected %v", args, expectedArgs)
	}
}
