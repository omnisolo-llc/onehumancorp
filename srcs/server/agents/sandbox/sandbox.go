package sandbox

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"regexp"
)

type SandboxManager struct {
	SandboxDir string
	blockedPatterns []*regexp.Regexp
}

func NewSandboxManager() (*SandboxManager, error) {
	dir, err := os.MkdirTemp("", "ohc_sandbox_*")
	if err != nil {
		return nil, err
	}
	if err := os.Chmod(dir, 0700); err != nil {
		os.RemoveAll(dir)
		return nil, err
	}
	return &SandboxManager{
		SandboxDir: dir,
		blockedPatterns: []*regexp.Regexp{
			regexp.MustCompile(`(?i)\bsudo\b`),
			regexp.MustCompile(`(?i)\brm\s+-rf\s+/`),
			regexp.MustCompile(`(?i)\bchown\b`),
			regexp.MustCompile(`(?i)\bchmod\b`),
			regexp.MustCompile(`(?i)\.git/(hooks|config)`),
			regexp.MustCompile(`<\(`),
			regexp.MustCompile(`>\(`),
			regexp.MustCompile(`=\(`),
		},
	}, nil
}



func (s *SandboxManager) ValidateContext(ctx context.Context, command string) error {
	for _, pattern := range s.blockedPatterns {
		if pattern.MatchString(command) {
			return fmt.Errorf("command violates security policy: matched %s", pattern.String())
		}
	}
	return nil
}

func (s *SandboxManager) Execute(ctx context.Context, cmdStr string) (string, error) {
	if err := s.ValidateContext(ctx, cmdStr); err != nil {
		return fmt.Sprintf("<sandbox_violations>%v</sandbox_violations>", err), err
	}

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
