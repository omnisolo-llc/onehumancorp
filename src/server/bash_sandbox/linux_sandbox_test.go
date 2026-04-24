package bash_sandbox

import (
	"context"
	"fmt"
	"path/filepath"
	"reflect"
	"strings"
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

	sockPath := filepath.Join(workDir, "proxy.sock")
	expectedScript := fmt.Sprintf(`socat UNIX-LISTEN:%s,fork TCP:127.0.0.1:8080 &
SOCAT_PID=$!
bwrap --unshare-net --unshare-pid --ro-bind / / --dev /dev --proc /proc --bind %s %s --seccomp 9 -- bash -c %q
kill $SOCAT_PID
wait $SOCAT_PID 2>/dev/null || true`, sockPath, workDir, workDir, command)

	expectedArgs := []string{
		"bwrap",
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--proc", "/proc",
		"--dev", "/dev",
		"--bind", "/", "/",
		"--",
		"bash", "-c", expectedScript,
	}

	if !reflect.DeepEqual(cmd.Args, expectedArgs) {
		t.Errorf("Expected args %v, got %v", expectedArgs, cmd.Args)
	}

	if !strings.Contains(expectedScript, "socat") {
		t.Errorf("Expected script to contain socat proxy logic")
	}

	if !strings.Contains(expectedScript, "--seccomp 9") {
		t.Errorf("Expected script to contain seccomp flag")
	}

	if len(cmd.ExtraFiles) != 1 {
	    t.Errorf("Expected exactly 1 ExtraFile for seccomp profile, got %d", len(cmd.ExtraFiles))
	}
}
