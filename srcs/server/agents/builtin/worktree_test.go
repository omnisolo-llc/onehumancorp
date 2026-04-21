package builtin

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
)

func TestWorktreeManager_CreateCleanup(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "worktree_test_*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	repoDir := filepath.Join(tempDir, "repo")
	err = os.MkdirAll(repoDir, 0755)
	if err != nil {
		t.Fatal(err)
	}

	// Change to repo dir
	originalWd, _ := os.Getwd()
	os.Chdir(repoDir)
	defer os.Chdir(originalWd)

	// Init git repo
	exec.Command("git", "init").Run()
	exec.Command("git", "config", "user.email", "test@test.com").Run()
	exec.Command("git", "config", "user.name", "Test").Run()
	os.WriteFile("test.txt", []byte("hello"), 0644)
	exec.Command("git", "add", "test.txt").Run()
	exec.Command("git", "commit", "-m", "init").Run()

	worktreesDir := filepath.Join(tempDir, ".ohc-worktrees")
	wm := NewWorktreeManager(worktreesDir)
	wm.repoDir = repoDir // explicitly set repo dir

	ctx := context.Background()
	taskID := "test-task-123"

	worktreePath, err := wm.Create(ctx, taskID)
	if err != nil {
		t.Fatalf("Create failed: %v", err)
	}

	expectedPath := filepath.Join(worktreesDir, taskID)
	if worktreePath != expectedPath {
		t.Errorf("expected path %s, got %s", expectedPath, worktreePath)
	}

	if _, err := os.Stat(filepath.Join(worktreePath, "test.txt")); os.IsNotExist(err) {
		t.Errorf("worktree does not contain repo files")
	}

	err = wm.Cleanup(ctx, taskID)
	if err != nil {
		t.Fatalf("Cleanup failed: %v", err)
	}

	if _, err := os.Stat(worktreePath); !os.IsNotExist(err) {
		t.Errorf("worktree directory still exists")
	}
}
