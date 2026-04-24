package harness

import (
	"context"
	"reflect"
	"strings"
	"testing"
)

func TestMacOsSandboxRunnerProfileGeneration(t *testing.T) {
	runner := NewMacOsSandboxRunner(nil)
	profile := runner.GenerateProfile(nil)

	expectedProfile := `(version 1)
(deny default)
(allow process-exec)
(allow process-fork)
(deny network*)
(allow file-read* (subpath "/"))
`
	if profile != expectedProfile {
		t.Errorf("GenerateProfile() = %q, want %q", profile, expectedProfile)
	}
}

func TestMacOsSandboxRunnerProfileGenerationWithPolicy(t *testing.T) {
	runner := NewMacOsSandboxRunner(nil)
	policy := &Policy{
		AllowNetwork: true,
		AllowedPaths: []string{"/Users/jules"},
		BlockedPaths: []string{"/etc/passwd"},
	}
	profile := runner.GenerateProfile(policy)

	expectedProfile := `(version 1)
(deny default)
(allow process-exec)
(allow process-fork)
(allow network*)
(allow file-read* (subpath "/"))
(allow file-write* (subpath "/Users/jules"))
(deny file* (subpath "/etc/passwd"))
`
	if profile != expectedProfile {
		t.Errorf("GenerateProfile() = %q, want %q", profile, expectedProfile)
	}
}

func TestMacOsSandboxRunnerArguments(t *testing.T) {
	runner := NewMacOsSandboxRunner(nil)
	command := `echo "hello"`
	args := runner.GetSandboxExecArgs(command, nil)

	expectedProfile := `(version 1)
(deny default)
(allow process-exec)
(allow process-fork)
(deny network*)
(allow file-read* (subpath "/"))
`
	expectedArgs := []string{"-p", expectedProfile, "bash", "-c", command}

	if !reflect.DeepEqual(args, expectedArgs) {
		t.Errorf("GetSandboxExecArgs() = %v, want %v", args, expectedArgs)
	}
}

func TestMacOsSandboxRunner_Execute_ValidationFailure(t *testing.T) {
	runner := NewMacOsSandboxRunner(nil)

	command := `echo "su"$(echo "do")`
	_, err := runner.Execute(context.Background(), command)

	if err == nil {
		t.Errorf("Expected validation to fail, but it passed")
	} else if !strings.HasPrefix(err.Error(), "command validation failed") {
		t.Errorf("Expected validation failure error, got: %v", err)
	}
}

func TestMacOsSandboxRunner_Execute_SandboxExecFailure(t *testing.T) {
	// Like bwrap runner, we skip execution test because sandbox-exec
	// might not be installed (e.g. on Linux build machines).
}
