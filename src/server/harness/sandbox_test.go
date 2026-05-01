package harness

import (
	"context"
	"os"
	"path/filepath"
	"runtime"
	"testing"
)

func TestSandboxExecution_ReadAllowed(t *testing.T) {
	if runtime.GOOS != "linux" && runtime.GOOS != "darwin" {
		t.Skip("Sandboxing only supported on Linux/macOS")
	}

	manager, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create sandbox manager: %v", err)
	}

	workDir := t.TempDir()

	// Should be able to read a public file like /etc/hosts
	cmd := []string{"cat", "/etc/hosts"}
	output, err := manager.Execute(context.Background(), cmd, workDir)
	if err != nil {
		t.Errorf("Failed to read /etc/hosts inside sandbox: %v, output: %s", err, string(output))
	}
}

func TestSandboxExecution_WriteBlockedOutsideWorkDir(t *testing.T) {
	if runtime.GOOS != "linux" && runtime.GOOS != "darwin" {
		t.Skip("Sandboxing only supported on Linux/macOS")
	}

	manager, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create sandbox manager: %v", err)
	}

	workDir := t.TempDir()

	// Attempt to write outside the work directory (e.g. /tmp/forbidden.txt)
	forbiddenFile := "/tmp/forbidden_sandbox_test.txt"
	defer os.Remove(forbiddenFile) // Clean up just in case

	cmd := []string{"sh", "-c", "echo 'hack' > " + forbiddenFile}
	_, err = manager.Execute(context.Background(), cmd, workDir)

	if err == nil {
		t.Errorf("Expected sandbox to block write outside workdir, but it succeeded")
	}
}

func TestSandboxExecution_WriteAllowedInsideWorkDir(t *testing.T) {
	if runtime.GOOS != "linux" && runtime.GOOS != "darwin" {
		t.Skip("Sandboxing only supported on Linux/macOS")
	}

	manager, err := NewSandboxManager()
	if err != nil {
		t.Fatalf("Failed to create sandbox manager: %v", err)
	}

	workDir := t.TempDir()
	allowedFile := filepath.Join(workDir, "allowed.txt")

	cmd := []string{"sh", "-c", "echo 'hello' > " + allowedFile}
	_, err = manager.Execute(context.Background(), cmd, workDir)

	if err != nil {
		t.Errorf("Expected sandbox to allow write inside workdir, but it failed: %v", err)
	}

	// Verify the file was actually written
	content, err := os.ReadFile(allowedFile)
	if err != nil {
		t.Errorf("Failed to read allowed file after sandbox execution: %v", err)
	}

	if string(content) != "hello\n" {
		t.Errorf("Expected file content 'hello', got '%s'", string(content))
	}
}
