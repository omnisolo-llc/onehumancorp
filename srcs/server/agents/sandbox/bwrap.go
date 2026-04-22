package sandbox

import (
	"context"
	"fmt"
	"os/exec"
	"sort"
)

// LinuxBwrapAdapter implements HarnessAdapter using bubblewrap.
type LinuxBwrapAdapter struct {
	BwrapPath string
}

// NewLinuxBwrapAdapter creates a new LinuxBwrapAdapter.
func NewLinuxBwrapAdapter() *LinuxBwrapAdapter {
	return &LinuxBwrapAdapter{
		BwrapPath: "bwrap",
	}
}

// BuildArgs constructs the bubblewrap command line arguments based on the config.
func (a *LinuxBwrapAdapter) BuildArgs(cmd string, cfg Config) []string {
	args := []string{
		"--unshare-pid",
		"--unshare-net",
		"--dev", "/dev",
	}

	// Sort bind mounts for deterministic argument order (especially useful for testing)
	var binds []string
	for hostPath := range cfg.Binds {
		binds = append(binds, hostPath)
	}
	sort.Strings(binds)
	for _, hostPath := range binds {
		args = append(args, "--bind", hostPath, cfg.Binds[hostPath])
	}

	var roBinds []string
	for hostPath := range cfg.RoBinds {
		roBinds = append(roBinds, hostPath)
	}
	sort.Strings(roBinds)
	for _, hostPath := range roBinds {
		args = append(args, "--ro-bind", hostPath, cfg.RoBinds[hostPath])
	}

	// Important: To properly pass dynamic arguments inside a bash wrapper without
	// relying on string interpolation vulnerabilities, we use 'bash -c' and pass
	// the actual command. However, since we are wrapping arbitrary shell commands
	// provided as a single string `cmd`, we'll pass it directly to `bash -c`.
	args = append(args, "--", "bash", "-c", cmd)

	return args
}

// Execute runs the command inside the bwrap sandbox.
func (a *LinuxBwrapAdapter) Execute(ctx context.Context, cmd string, cfg Config) (*Result, error) {
	args := a.BuildArgs(cmd, cfg)

	execCmd := exec.CommandContext(ctx, a.BwrapPath, args...)

	// When running via bwrap, we need a PATH so it can find bash and other utilities if needed.
	execCmd.Env = append(execCmd.Environ(), "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin")

	out, err := execCmd.CombinedOutput()
	if err != nil {
		return &Result{Output: string(out)}, fmt.Errorf("bwrap execution failed: %w", err)
	}

	return &Result{Output: string(out)}, nil
}
