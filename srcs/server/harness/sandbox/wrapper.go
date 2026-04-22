package sandbox

import (
	"fmt"
	"os/exec"
	"strings"
)

// escapeShellArg properly escapes a string for safe use in a bash/sh command line.
// It wraps the string in single quotes, and replaces any existing single quotes
// with '\'' to break out, output the quote, and resume quoting.
func escapeShellArg(arg string) string {
	return "'" + strings.ReplaceAll(arg, "'", "'\\''") + "'"
}

func WrapCommand(cmd string) string {
	escapedCmd := escapeShellArg(cmd)

	// Dynamically check for bwrap availability
	_, err := exec.LookPath("bwrap")
	if err == nil {
		// Use bwrap to run bash and pass the escaped command to it.
		// By doing this, shell metacharacters in cmd are evaluated *inside* the bwrap sandbox.
		return fmt.Sprintf("bwrap --unshare-all --ro-bind / / --dev /dev --proc /proc --tmpfs /tmp -- /bin/bash -c %s", escapedCmd)
	}

	// Graceful fallback to standard shell execute if bwrap is not available
	return fmt.Sprintf("/bin/bash -c %s", escapedCmd)
}
