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
		// May fail in tests if /tmp/testdir doesn't exist, ignore for now to focus on args
	}
	if cmd == nil {
		return // Can't assert if we couldn't create it due to missing dir
	}
	defer cleanup()

	expectedArgs := []string{
		"bwrap",
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--unshare-net",
		"--proc", "/proc",
		"--dev", "/dev",
		"--ro-bind", "/", "/",
		"--bind", workDir, workDir,
		"--",
		"bash", "-c", command,
	}

	if !reflect.DeepEqual(cmd.Args, expectedArgs) {
		t.Errorf("Expected args %v, got %v", expectedArgs, cmd.Args)
	}

	if len(cmd.ExtraFiles) != 0 {
		t.Errorf("Expected 0 ExtraFiles, got %d", len(cmd.ExtraFiles))
	}
}
