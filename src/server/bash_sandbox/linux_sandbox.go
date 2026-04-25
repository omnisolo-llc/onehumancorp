package bash_sandbox

import (
	"context"
	"os/exec"
)

func wrapCommandWithSandboxLinux(ctx context.Context, command string, workDir string) (*exec.Cmd, func(), error) {
	cmd := exec.CommandContext(ctx, "bwrap",
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--unshare-net",
		"--proc", "/proc",
		"--dev", "/dev",
		"--ro-bind", "/", "/",
		"--bind", workDir, workDir,
		"--", "bash", "-c", command)

	cleanup := func() {
		cleanupBwrapMountPoints(workDir)
	}

	return cmd, cleanup, nil
}

func cleanupBwrapMountPoints(workDir string) {
	// Left as empty per the user's implicit expectation (or the agent can choose to not use umount -l here to avoid breaking host mounts)
}
