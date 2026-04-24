package sandbox

import (
	"context"
	"fmt"
	"os"
	"os/exec"
)

// added for issue 5417
type SandboxManager struct {
	SandboxDir string
}

func NewSandboxManager(sessionID string) (*SandboxManager, error) {
	dir := fmt.Sprintf("/tmp/ohc-agent-sessions/%s", sessionID)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return nil, err
	}
	if err := os.Chmod(dir, 0700); err != nil {
		os.RemoveAll(dir)
		return nil, err
	}
	return &SandboxManager{SandboxDir: dir}, nil
}

func (s *SandboxManager) Execute(ctx context.Context, cmdStr string) (string, error) {
	wrapperCmd := fmt.Sprintf("shopt -u extglob 2>/dev/null || true; %s", cmdStr)
	cmd := exec.CommandContext(ctx, "bash", "-c", wrapperCmd)
	cmd.Dir = s.SandboxDir
	cmd.Env = []string{fmt.Sprintf("TMPDIR=%s", s.SandboxDir), "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"}

	out, err := cmd.CombinedOutput()
	return string(out), err
}

func (s *SandboxManager) Cleanup() error {
	return os.RemoveAll(s.SandboxDir)
}
