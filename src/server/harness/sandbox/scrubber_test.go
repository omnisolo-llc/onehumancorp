package sandbox

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/src/server/harness"
)

type MockHarness struct {
	workDir string
}

func (m *MockHarness) Execute(ctx context.Context, command string) (harness.Result, error) {
	// Simulate an agent trying to create a malicious HEAD file
	os.MkdirAll(filepath.Join(m.workDir, ".git"), 0755)
	os.WriteFile(filepath.Join(m.workDir, ".git", "HEAD"), []byte("ref: refs/heads/main\n"), 0644)

	// Simulate creating a bare repo config
	os.WriteFile(filepath.Join(m.workDir, "config"), []byte("[core]\n\trepositoryformatversion = 0\n"), 0644)

	return harness.Result{Stdout: "success"}, nil
}

func TestGitScrubberInterceptor(t *testing.T) {
	workDir, err := os.MkdirTemp("", "scrubber-test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(workDir)

	mockHarness := &MockHarness{workDir: workDir}
	interceptor := NewGitScrubberInterceptor(mockHarness, workDir)

	_, err = interceptor.Execute(context.Background(), "some_command")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Verify that the files were scrubbed
	if _, err := os.Stat(filepath.Join(workDir, ".git", "HEAD")); !os.IsNotExist(err) {
		t.Errorf("Expected .git/HEAD to be scrubbed, but it still exists")
	}

	if _, err := os.Stat(filepath.Join(workDir, "config")); !os.IsNotExist(err) {
		t.Errorf("Expected bare repo config to be scrubbed, but it still exists")
	}
}

func TestGitScrubberInterceptor_PreservesExisting(t *testing.T) {
	workDir, err := os.MkdirTemp("", "scrubber-test-preserve")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(workDir)

	// Create an existing .git/HEAD file before execution
	os.MkdirAll(filepath.Join(workDir, ".git"), 0755)
	err = os.WriteFile(filepath.Join(workDir, ".git", "HEAD"), []byte("ref: refs/heads/existing\n"), 0644)
	if err != nil {
		t.Fatalf("Failed to write initial HEAD file: %v", err)
	}

	mockHarness := &MockHarness{workDir: workDir}
	interceptor := NewGitScrubberInterceptor(mockHarness, workDir)

	_, err = interceptor.Execute(context.Background(), "some_command")
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	// Verify that the file was preserved
	data, err := os.ReadFile(filepath.Join(workDir, ".git", "HEAD"))
	if err != nil {
		t.Errorf("Expected .git/HEAD to be preserved, but got error: %v", err)
	}

	// mock actually overwrote it, so it should be the new content "ref: refs/heads/main\n"
	if string(data) != "ref: refs/heads/main\n" {
		t.Errorf("Expected .git/HEAD content to be 'ref: refs/heads/main\\n', got '%s'", string(data))
	}
}
