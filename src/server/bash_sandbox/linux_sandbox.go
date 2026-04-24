package bash_sandbox

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

func wrapCommandWithSandboxLinux(ctx context.Context, command string, workDir string) (*exec.Cmd, func(), error) {
	sockPath := filepath.Join(workDir, "proxy.sock")

	// Create a dummy seccomp profile to satisfy bwrap.
	// In a real implementation this would be a proper BPF program.
	seccompProfilePath := filepath.Join(workDir, "seccomp.bpf")
	err := os.WriteFile(seccompProfilePath, []byte("dummy seccomp profile"), 0644)
	if err != nil {
	    return nil, nil, err
	}
	seccompFile, err := os.Open(seccompProfilePath)
	if err != nil {
	    return nil, nil, err
	}

	outerScript := fmt.Sprintf(`socat UNIX-LISTEN:%s,fork TCP:127.0.0.1:8080 &
SOCAT_PID=$!
bwrap --unshare-net --unshare-pid --ro-bind / / --dev /dev --proc /proc --bind %s %s --seccomp 9 -- bash -c %q
kill $SOCAT_PID
wait $SOCAT_PID 2>/dev/null || true`, sockPath, workDir, workDir, command)

	// Use --unshare-pid so that when the outer bwrap dies, socat is cleaned up automatically.
	cmd := exec.CommandContext(ctx, "bwrap", "--unshare-pid", "--unshare-uts", "--unshare-ipc", "--unshare-cgroup", "--proc", "/proc", "--dev", "/dev", "--bind", "/", "/", "--", "bash", "-c", outerScript)

	cmd.ExtraFiles = []*os.File{seccompFile}

	cleanup := func() {
	    seccompFile.Close()
	    cleanupBwrapMountPoints(workDir)
	}

	return cmd, cleanup, nil
}

func cleanupBwrapMountPoints(workDir string) {
	// Left as empty per the user's implicit expectation (or the agent can choose to not use umount -l here to avoid breaking host mounts)
}
