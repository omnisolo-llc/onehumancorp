package sandbox

import (
	"bytes"
	"context"
	"fmt"
	"os"
	"os/exec"
)

type ShellProvider interface {
	Execute(ctx context.Context, cmd string) (string, string, error)
}

type BashProvider struct {
	Manager *SandboxManager
}

func (b *BashProvider) Execute(ctx context.Context, cmd string) (string, string, error) {
	return b.Manager.Execute(ctx, cmd)
}

type PowerShellProvider struct {
	Manager *SandboxManager
}

func (p *PowerShellProvider) Execute(ctx context.Context, command string) (string, string, error) {
	// For PowerShell, we use a basic execution model wrapped via the manager's tmpdir
	cmd := exec.CommandContext(ctx, "pwsh", "-Command", command)

	var env []string
	for _, e := range os.Environ() {
		if len(e) >= 7 && e[:7] == "TMPDIR=" {
			continue
		}
		env = append(env, e)
	}
	env = append(env, fmt.Sprintf("TMPDIR=%s", p.Manager.tmpDir))
	cmd.Env = env

	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()

	return stdout.String(), stderr.String(), err
}
