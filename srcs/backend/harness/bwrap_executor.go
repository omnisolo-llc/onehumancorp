package harness

import (
	"context"
	"os/exec"
)

type BwrapExecutor struct {
	ProxySocketPath string
}

func NewBwrapExecutor(proxySocketPath string) *BwrapExecutor {
	return &BwrapExecutor{
		ProxySocketPath: proxySocketPath,
	}
}

func (e *BwrapExecutor) Execute(ctx context.Context, cmd string, args ...string) ([]byte, error) {
	bwrapArgs := []string{
		"--ro-bind", "/", "/",
		"--tmpfs", "/tmp",
		"--unshare-all",
	}

	if e.ProxySocketPath != "" {
		// If proxy socket is provided, map it in and configure network namespace
		bwrapArgs = append(bwrapArgs,
			"--unshare-net",
			"--bind", e.ProxySocketPath, e.ProxySocketPath,
			"--setenv", "HTTP_PROXY", "http://127.0.0.1:3128",
			"--setenv", "HTTPS_PROXY", "http://127.0.0.1:3128",
			"--setenv", "ALL_PROXY", "http://127.0.0.1:3128",
			"--",
			"bash", "-c",
			// Start inner socat and then execute the requested command.
			// Pass ProxySocketPath as $1 to avoid bash injection.
			"ip link set dev lo up || true; socat TCP-LISTEN:3128,fork UNIX-CLIENT:\"$1\" < /dev/null & sleep 0.1; shift; exec \"$@\"",
			"--",
			e.ProxySocketPath,
			cmd,
		)
	} else {
		bwrapArgs = append(bwrapArgs,
			"--share-net",
			"--",
			cmd,
		)
	}

	bwrapArgs = append(bwrapArgs, args...)

	execCmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)
	return execCmd.CombinedOutput()
}
