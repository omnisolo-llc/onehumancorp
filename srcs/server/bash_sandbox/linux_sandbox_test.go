package bash_sandbox

import (
	"context"
	"reflect"
	"testing"
)

func TestWrapCommandWithSandboxLinux(t *testing.T) {
	ctx := context.Background()
	workDir := "/tmp/testdir"
	command := "echo hello"

	cmd, cleanup, err := wrapCommandWithSandboxLinux(ctx, command, workDir)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	if cmd == nil {
		t.Fatalf("Expected cmd to not be nil")
	}
	if cleanup == nil {
		t.Fatalf("Expected cleanup to not be nil")
	}

	expectedArgs := []string{
		"bwrap",
		"--unshare-net",
		"--unshare-pid",
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--bind", workDir, workDir,
		"--",
		"bash", "-c", command,
	}

	if !reflect.DeepEqual(cmd.Args, expectedArgs) {
		t.Errorf("Expected args %v, got %v", expectedArgs, cmd.Args)
	}
}
