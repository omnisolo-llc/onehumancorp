package harness

import (
	"context"
	"fmt"
	"os/exec"
	"srcs/server/harness/sandbox"
)

type LocalShellTask struct {
	manager *sandbox.SandboxManager
}

func NewLocalShellTask() *LocalShellTask {
	return &LocalShellTask{
		manager: sandbox.GetSandboxManager(),
	}
}

func (l *LocalShellTask) Execute(ctx context.Context, cmd string) (string, error) {
	wrappedCmd, err := l.manager.WrapCommand(ctx, cmd)
	if err != nil {
		return "", fmt.Errorf("%s", l.manager.AnnotateError(err, ""))
	}

	// Example simplified execution for intercept testing
	execCmd := exec.CommandContext(ctx, "bash", "-c", wrappedCmd)
	out, err := execCmd.CombinedOutput()
	if err != nil {
		return string(out), fmt.Errorf("%s", l.manager.AnnotateError(err, string(out)))
	}

	return string(out), nil
}
