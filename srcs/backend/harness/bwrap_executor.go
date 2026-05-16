package harness

import (
	"bytes"
	"os/exec"
)

// BwrapExecutor executes commands within a Bubblewrap (bwrap) sandbox.
type BwrapExecutor struct{}

// NewBwrapExecutor creates a new BwrapExecutor.
func NewBwrapExecutor() *BwrapExecutor {
	return &BwrapExecutor{}
}

// Execute wraps the given command with bwrap to isolate filesystem access.
func (b *BwrapExecutor) Execute(cmd string) (string, error) {
	bwrapArgs := []string{
		"--unshare-pid",
		"--unshare-ipc",
		"--unshare-uts",
		"--unshare-net",
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--tmpfs", "/tmp",
		"--",
		"bash",
		"-c",
		cmd,
	}

	c := exec.Command("bwrap", bwrapArgs...)
	var out bytes.Buffer
	var stderr bytes.Buffer
	c.Stdout = &out
	c.Stderr = &stderr

	err := c.Run()
	if err != nil {
		return stderr.String(), err
	}
	return out.String(), nil
}
