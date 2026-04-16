package builtin

import (
	"bytes"
	"context"
	"fmt"
	"os/exec"
	"time"
)

// SubprocessHelper executes shell commands with a timeout and captures standard output and error.
// Mirrors CC-Source shell execution primitives.
func SubprocessHelper(ctx context.Context, command string, timeout time.Duration) (string, error) {
	ctx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()

	cmd := exec.CommandContext(ctx, "bash", "-c", command)
	var outbuf, errbuf bytes.Buffer
	cmd.Stdout = &outbuf
	cmd.Stderr = &errbuf

	err := cmd.Run()
	if ctx.Err() == context.DeadlineExceeded {
		return "", fmt.Errorf("subprocess timed out after %v", timeout)
	}
	if err != nil {
		return "", fmt.Errorf("subprocess error: %w, stderr: %s", err, errbuf.String())
	}

	return outbuf.String(), nil
}
