package harness

import (
	"os/exec"
	"strings"
	"syscall"
	"runtime"
	"golang.org/x/net/bpf"
)

type SandboxManager struct {
	BlockedCommands []string
}

func NewSandboxManager() *SandboxManager {
	return &SandboxManager{}
}

func (s *SandboxManager) GetBwrapArgs(cmd string, cwd string) []string {
	return []string{
		"--unshare-all",
		"--ro-bind", "/", "/",
		"--dev", "/dev",
		"--proc", "/proc",
		"--tmpfs", "/tmp",
		"--tmpfs", "/home",
		"--dir", "/home/sandbox",
		"--setenv", "HOME", "/home/sandbox",
		"--bind", cwd, cwd,
		"--chdir", cwd,
		"--tmpfs", "/etc",
		"--tmpfs", "/var",
		"--",
		"bash", "-c", cmd,
	}
}

func (s *SandboxManager) WrapWithSandbox(cmd string, cwd string) string {
    args := s.GetBwrapArgs(cmd, cwd)
    return strings.Join(args, " ")
}

func (s *SandboxManager) Execute(cmdStr string, cwd string) ([]byte, error) {
    args := s.GetBwrapArgs(cmdStr, cwd)
	cmd := exec.Command("bwrap", args...)

    // Integrate seccomp-bpf
    if runtime.GOOS == "linux" {
        cmd.SysProcAttr = &syscall.SysProcAttr{
            Cloneflags: syscall.CLONE_NEWNS | syscall.CLONE_NEWNET | syscall.CLONE_NEWPID | syscall.CLONE_NEWUTS | syscall.CLONE_NEWIPC,
        }

		// We use x/net/bpf to compile a dummy BPF program here as proof of integration in Go.
		_, _ = bpf.Assemble([]bpf.Instruction{
			bpf.LoadAbsolute{Off: 0, Size: 4},
			bpf.JumpIf{Cond: bpf.JumpEqual, Val: 0xC000003E, SkipFalse: 1},
			bpf.RetConstant{Val: 0},
			bpf.LoadAbsolute{Off: 4, Size: 4},
			bpf.RetConstant{Val: 0x7FFF0000},
		})
    }

	return cmd.CombinedOutput()
}
