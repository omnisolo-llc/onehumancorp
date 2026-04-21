package harness

import (
	"context"
	"os"
	"os/exec"
	"runtime"
	"strings"
)

type HarnessManager struct {
	store *ViolationStore
}

func NewHarnessManager() *HarnessManager {
	return &HarnessManager{
		store: NewViolationStore(),
	}
}

func (m *HarnessManager) GetViolations() []Violation {
	return m.store.GetViolations()
}

// WrapCommand prepares the command array for execution using bwrap or sandbox-exec
// Instead of building a string, it builds the argument list.
func (m *HarnessManager) WrapCommand(ctx context.Context, cmd string) []string {
	if err := ValidateCommand(cmd); err != nil {
		m.store.RecordViolation(ctx, cmd, err.Error())
	}

	if runtime.GOOS == "darwin" {
		return []string{"sandbox-exec", "-n", "no-network", "bash", "-c", cmd}
	} else if runtime.GOOS == "linux" {
		return []string{"bwrap", "--unshare-net", "--ro-bind", "/", "/", "--", "bash", "-c", cmd}
	}
	return []string{"bash", "-c", cmd}
}

func (m *HarnessManager) Execute(ctx context.Context, cmd string) (string, string, error) {
	if err := ValidateCommand(cmd); err != nil {
		m.store.RecordViolation(ctx, cmd, err.Error())
		// Only annotate with the current violation if we fail right away
		currentFailures := []Violation{{Command: cmd, Error: err.Error()}}
		return "", AnnotateStderrWithSandboxFailures("", currentFailures), err
	}

	args := m.WrapCommand(ctx, cmd)
	c := exec.CommandContext(ctx, args[0], args[1:]...)
	c.Env = ScrubEnv(os.Environ())

	var stdout strings.Builder
	var stderr strings.Builder
	c.Stdout = &stdout
	c.Stderr = &stderr
	err := c.Run()

	stderrStr := stderr.String()
	// Only annotate if we recorded violations during this run, but since we already validate,
	// if it fails later, it's not a validation error. If we want to record runtime bwrap errors:
	// This would require parsing bwrap output, but for now we just return standard stderr.
	return stdout.String(), stderrStr, err
}
