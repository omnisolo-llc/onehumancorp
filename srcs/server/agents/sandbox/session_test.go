package sandbox

import (
	"context"
	"path/filepath"
	"testing"
)

func TestShellSession_EnvState(t *testing.T) {
	tempDir := t.TempDir()
	session, err := NewShellSession("test_session_1", tempDir)
	if err != nil {
		t.Fatalf("Failed to create session: %v", err)
	}

	ctx := context.Background()

	// 1. Set environment variable
	out, err := session.RunStatefulCommand(ctx, "export FOO=bar")
	if err != nil {
		t.Fatalf("Failed to run export: %v, out: %s", err, out)
	}

	// 2. Read environment variable in next command
	out, err = session.RunStatefulCommand(ctx, "echo $FOO")
	if err != nil {
		t.Fatalf("Failed to run echo: %v, out: %s", err, out)
	}

	if string(out) != "bar\n" {
		t.Errorf("Expected 'bar\\n', got '%s'", out)
	}
}

func TestShellSession_CwdState(t *testing.T) {
	tempDir := t.TempDir()
	session, err := NewShellSession("test_session_2", tempDir)
	if err != nil {
		t.Fatalf("Failed to create session: %v", err)
	}

	ctx := context.Background()

	// 1. Create directory and change into it
	out, err := session.RunStatefulCommand(ctx, "mkdir -p test_dir && cd test_dir")
	if err != nil {
		t.Fatalf("Failed to run mkdir/cd: %v, out: %s", err, out)
	}

	// 2. Read working directory in next command
	out, err = session.RunStatefulCommand(ctx, "pwd")
	if err != nil {
		t.Fatalf("Failed to run pwd: %v, out: %s", err, out)
	}

	expectedDir := filepath.Join(tempDir, "test_dir")
	// The out might have a trailing newline
	actualDir := out
	if len(actualDir) > 0 && actualDir[len(actualDir)-1] == '\n' {
		actualDir = actualDir[:len(actualDir)-1]
	}

	if actualDir != expectedDir {
		t.Errorf("Expected '%s', got '%s'", expectedDir, actualDir)
	}
}
