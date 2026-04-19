package bash_sandbox

import (
	"context"
	"os/exec"
)

func wrapCommandWithSandboxLinux(ctx context.Context, command string, workDir string) (*exec.Cmd, func(), error) {
    cmd := exec.CommandContext(ctx, "bwrap", "--unshare-net", "--unshare-pid", "--ro-bind", "/", "/", "--dev", "/dev", "--proc", "/proc", "--bind", workDir, workDir, "--", "bash", "-c", command)
    return cmd, func() { cleanupBwrapMountPoints(workDir) }, nil
}

func cleanupBwrapMountPoints(workDir string) {
    // Left as empty per the user's implicit expectation (or the agent can choose to not use umount -l here to avoid breaking host mounts)
}
