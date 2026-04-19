package harness

import (
	"context"
	"reflect"
	"strings"
	"testing"
)

func TestBwrapRunnerArguments(t *testing.T) {
	runner := NewBwrapRunner(nil)
	command := `echo "hello world"`
	args := runner.GetBwrapArgs(command, nil)

	expectedArgs := []string{
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--proc", "/proc",
		"--dev", "/dev",
		"--tmpfs", "/tmp",
		"--unshare-net",
		"--ro-bind", "/", "/",
		"--bind", "/var/run/ohc_proxy.sock", "/var/run/ohc_proxy.sock",
		"--",
		"bash", "-c", command,
	}

	if !reflect.DeepEqual(args, expectedArgs) {
		t.Errorf("GetBwrapArgs() = %v, want %v", args, expectedArgs)
	}
}

func TestBwrapRunnerArgumentsWithPolicy(t *testing.T) {
	runner := NewBwrapRunner(nil)
	command := `echo "hello world"`
	policy := &Policy{
		AllowNetwork: true,
		AllowedPaths: []string{"/home/user"},
		ReadOnlyPaths: []string{"/etc/config"},
	}
	args := runner.GetBwrapArgs(command, policy)

	// Note: We don't check for --unshare-net
	expectedArgs := []string{
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--proc", "/proc",
		"--dev", "/dev",
		"--tmpfs", "/tmp",
		"--ro-bind", "/", "/",
		"--bind", "/home/user", "/home/user",
		"--ro-bind", "/etc/config", "/etc/config",
		"--bind", "/var/run/ohc_proxy.sock", "/var/run/ohc_proxy.sock",
		"--",
		"bash", "-c", command,
	}

	if !reflect.DeepEqual(args, expectedArgs) {
		t.Errorf("GetBwrapArgs() = %v, want %v", args, expectedArgs)
	}
}

func TestBwrapRunner_Execute_ValidationFailure(t *testing.T) {
	runner := NewBwrapRunner(nil)

	// This command should fail AST validation
	command := `echo "su"$(echo "do")`
	_, err := runner.Execute(context.Background(), command)

	if err == nil {
		t.Errorf("Expected validation to fail, but it passed")
	} else if !strings.HasPrefix(err.Error(), "command validation failed") {
		t.Errorf("Expected validation failure error, got: %v", err)
	}
}

func TestBwrapRunner_Execute_BwrapFailure(t *testing.T) {
	// We can't easily trigger a bwrap failure without bwrap installed,
	// but we can try to run it and expect "file not found" or similar if we use a fake path
	// but BwrapRunner uses exec.CommandContext("bwrap", ...) directly.
}
