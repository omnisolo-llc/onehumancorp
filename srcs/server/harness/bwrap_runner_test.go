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
	args := runner.GetBwrapArgs(command)

	expectedArgs := []string{
		"--unshare-net",
		"--unshare-pid",
		"--unshare-uts",
		"--unshare-ipc",
		"--unshare-cgroup",
		"--ro-bind", "/", "/",
		"--proc", "/proc",
		"--dev", "/dev",
		"--tmpfs", "/tmp",
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
