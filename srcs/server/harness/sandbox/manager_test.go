package sandbox

import (
	"context"
	"errors"
	"testing"
	"strings"
	"github.com/onehumancorp/mono/srcs/server/harness"
)

func TestSandboxManager_WrapCommand(t *testing.T) {
	eval := NewPermissionEvaluator([]string{}, []string{".*"})
	sm := NewSandboxManager(harness.Config{}, eval)
	cmd := "echo 'hello' && rm -rf /"
	wrapped, err := sm.WrapCommand(context.Background(), cmd)
	if err != nil {
		t.Fatalf("WrapCommand failed: %v", err)
	}

	// Check it escapes correctly
	if !strings.Contains(wrapped, "'echo '\\''hello'\\'' && rm -rf /'") {
		t.Errorf("Expected wrapped string to contain properly escaped command, got: %v", wrapped)
	}
}

func TestSandboxManager_WrapCommand_Denied(t *testing.T) {
	eval := NewPermissionEvaluator([]string{"bad_cmd"}, nil)
	sm := NewSandboxManager(harness.Config{}, eval)

	_, err := sm.WrapCommand(context.Background(), "bad_cmd")
	if err == nil {
		t.Errorf("Expected command to be denied by policy")
	}
}

func TestSandboxManager_AnnotateError(t *testing.T) {
	sm := NewSandboxManager(harness.Config{}, nil)
	err := errors.New("command failed")
	stdout := "some output"
	annotated := sm.AnnotateError(err, stdout)
	expected := "Sandbox Violation: command failed\nStdout: some output"
	if annotated != expected {
		t.Errorf("Expected %q, got %q", expected, annotated)
	}
}
