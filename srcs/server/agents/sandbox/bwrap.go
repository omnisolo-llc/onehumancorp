package sandbox

import (
	"bytes"
	"context"
	"os/exec"
	"sort"
)

// LinuxBwrapAdapter implements the HarnessAdapter interface using bwrap.
type LinuxBwrapAdapter struct{}

// BuildBwrapArgs builds the argument list for the bwrap command based on the configuration.
// It is exported so that it can be unit-tested without actually running bwrap.
func (a *LinuxBwrapAdapter) BuildBwrapArgs(cmdStr string, cfg Config) []string {
	args := []string{
		"--unshare-pid",
		"--unshare-net",
		"--dev", "/dev",
	}

	var bindKeys []string
	for k := range cfg.Bind {
		bindKeys = append(bindKeys, k)
	}
	sort.Strings(bindKeys)
	for _, hostPath := range bindKeys {
		args = append(args, "--bind", hostPath, cfg.Bind[hostPath])
	}

	var roBindKeys []string
	for k := range cfg.RoBind {
		roBindKeys = append(roBindKeys, k)
	}
	sort.Strings(roBindKeys)
	for _, hostPath := range roBindKeys {
		args = append(args, "--ro-bind", hostPath, cfg.RoBind[hostPath])
	}

	// Finally, append the actual command to run. We use bash -c to run the string.
	args = append(args, "bash", "-c", cmdStr)
	return args
}

// limitedWriter wraps a bytes.Buffer and caps the total number of bytes written.
type limitedWriter struct {
	buf   bytes.Buffer
	limit int64
	n     int64
}

func (w *limitedWriter) Write(p []byte) (n int, err error) {
	if w.n >= w.limit {
		return len(p), nil // drop bytes beyond limit but pretend they were written
	}
	remaining := w.limit - w.n
	toWrite := int64(len(p))
	if toWrite > remaining {
		toWrite = remaining
	}
	written, err := w.buf.Write(p[:toWrite])
	w.n += int64(written)
	// Even if we truncated the write because of the limit,
	// we return len(p) so the caller (like exec.Cmd) doesn't fail with "short write"
	return len(p), err
}

// Execute safely runs a shell command using bwrap for isolation.
func (a *LinuxBwrapAdapter) Execute(ctx context.Context, cmdStr string, cfg Config) (*Result, error) {
	args := a.BuildBwrapArgs(cmdStr, cfg)

	cmd := exec.CommandContext(ctx, "bwrap", args...)

	// Cap standard output and standard error at 10 MB each to prevent memory exhaustion
	const maxOutputSize = 10 * 1024 * 1024
	stdout := &limitedWriter{limit: maxOutputSize}
	stderr := &limitedWriter{limit: maxOutputSize}

	cmd.Stdout = stdout
	cmd.Stderr = stderr

	err := cmd.Run()

	res := &Result{
		Stdout: stdout.buf.String(),
		Stderr: stderr.buf.String(),
	}

	return res, err
}
