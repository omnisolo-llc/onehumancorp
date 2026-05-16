package harness

import (
	"fmt"
	"os/exec"
	"runtime"
	"strings"
)

// SandboxManager defines the interface for an OS-level sandbox wrapper
// to restrict filesystem and network access for agent tool execution.
type SandboxManager interface {
	// WrapCommand takes a base command and its arguments and returns the
	// sandbox-wrapped command and arguments.
	WrapCommand(command string, args []string) (string, []string, error)
	// ExecuteCommand wraps and executes the command.
	ExecuteCommand(command string, args []string) ([]byte, error)
}

// SandboxConfig holds configuration for the sandbox wrapper.
type SandboxConfig struct {
	WorkspaceDir string
	ReadOnlyDirs []string
	Network      bool // Whether to allow network access
}

// BwrapAdapter implements SandboxManager using bubblewrap (bwrap) on Linux.
type BwrapAdapter struct {
	Config SandboxConfig
}

func (b *BwrapAdapter) WrapCommand(command string, args []string) (string, []string, error) {
	// Instead of mounting the entire host root, we selectively mount
	// necessary system directories as read-only to prevent reading host secrets
	// or other tenant data.
	bwrapArgs := []string{
		"--ro-bind", "/bin", "/bin",
		"--ro-bind", "/usr", "/usr",
		"--ro-bind", "/lib", "/lib",
		"--ro-bind-try", "/lib64", "/lib64",
		"--ro-bind", "/etc/alternatives", "/etc/alternatives",
		"--dev", "/dev",
		"--proc", "/proc",
		"--tmpfs", "/tmp", // Isolated temporary folder
	}

	if b.Config.WorkspaceDir != "" {
		// Bind workspace dir as read-write
		bwrapArgs = append(bwrapArgs, "--bind", b.Config.WorkspaceDir, b.Config.WorkspaceDir)
	}

	for _, roDir := range b.Config.ReadOnlyDirs {
		bwrapArgs = append(bwrapArgs, "--ro-bind", roDir, roDir)
	}

	if b.Config.Network {
		bwrapArgs = append(bwrapArgs, "--ro-bind-try", "/etc/resolv.conf", "/etc/resolv.conf")
	} else {
		bwrapArgs = append(bwrapArgs, "--unshare-net")
	}
    bwrapArgs = append(bwrapArgs, "--unshare-pid")
    bwrapArgs = append(bwrapArgs, "--unshare-ipc")

	bwrapArgs = append(bwrapArgs, "--")
	bwrapArgs = append(bwrapArgs, command)
	bwrapArgs = append(bwrapArgs, args...)

	return "bwrap", bwrapArgs, nil
}

func (b *BwrapAdapter) ExecuteCommand(command string, args []string) ([]byte, error) {
	cmdName, cmdArgs, err := b.WrapCommand(command, args)
	if err != nil {
		return nil, err
	}
	cmd := exec.Command(cmdName, cmdArgs...)
	return cmd.CombinedOutput()
}

// SandboxExecAdapter implements SandboxManager using sandbox-exec on macOS.
type SandboxExecAdapter struct {
	Config SandboxConfig
}

func (s *SandboxExecAdapter) WrapCommand(command string, args []string) (string, []string, error) {
	// Construct a basic sandbox profile
	var profileBuilder strings.Builder
	profileBuilder.WriteString("(version 1)\n")
	profileBuilder.WriteString("(deny default)\n")
	profileBuilder.WriteString("(allow process-exec (regex #\"^/bin/.*\") (regex #\"^/usr/bin/.*\") (regex #\"^/usr/local/bin/.*\"))\n")
    profileBuilder.WriteString("(allow process-fork)\n")

	// Restrict read access to essentials and workspace to prevent data leakage
	profileBuilder.WriteString("(allow file-read* (subpath \"/bin\"))\n")
	profileBuilder.WriteString("(allow file-read* (subpath \"/usr\"))\n")
	profileBuilder.WriteString("(allow file-read* (subpath \"/System\"))\n")
	profileBuilder.WriteString("(allow file-read* (subpath \"/Library\"))\n")

	if s.Config.WorkspaceDir != "" {
		profileBuilder.WriteString(fmt.Sprintf("(allow file-read* (subpath \"%s\"))\n", s.Config.WorkspaceDir))
		profileBuilder.WriteString(fmt.Sprintf("(allow file-write* (subpath \"%s\"))\n", s.Config.WorkspaceDir))
	}
	profileBuilder.WriteString("(allow file-read* (subpath \"/tmp\"))\n")
	profileBuilder.WriteString("(allow file-write* (subpath \"/tmp\"))\n")
	profileBuilder.WriteString("(allow file-read* (subpath \"/private/tmp\"))\n")
	profileBuilder.WriteString("(allow file-write* (subpath \"/private/tmp\"))\n")
    profileBuilder.WriteString("(allow file-read* (subpath \"/var/folders\"))\n")
    profileBuilder.WriteString("(allow file-write* (subpath \"/var/folders\"))\n")
    profileBuilder.WriteString("(allow file-read* (subpath \"/private/var/folders\"))\n")
    profileBuilder.WriteString("(allow file-write* (subpath \"/private/var/folders\"))\n")

	if s.Config.Network {
		profileBuilder.WriteString("(allow network*)\n")
	}

	sandboxArgs := []string{"-p", profileBuilder.String(), command}
	sandboxArgs = append(sandboxArgs, args...)

	return "sandbox-exec", sandboxArgs, nil
}

func (s *SandboxExecAdapter) ExecuteCommand(command string, args []string) ([]byte, error) {
	cmdName, cmdArgs, err := s.WrapCommand(command, args)
	if err != nil {
		return nil, err
	}
	cmd := exec.Command(cmdName, cmdArgs...)
	return cmd.CombinedOutput()
}

// NewSandboxManager creates a new SandboxManager based on the operating system.
func NewSandboxManager(config SandboxConfig) (SandboxManager, error) {
	if runtime.GOOS == "linux" {
		return &BwrapAdapter{Config: config}, nil
	} else if runtime.GOOS == "darwin" {
		return &SandboxExecAdapter{Config: config}, nil
	}
	return nil, fmt.Errorf("unsupported operating system for sandbox: %s", runtime.GOOS)
}
