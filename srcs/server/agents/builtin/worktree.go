package builtin

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

// WorktreeManager manages isolated git worktrees for tasks.
type WorktreeManager struct {
	baseDir string
	repoDir string
}

// NewWorktreeManager creates a new WorktreeManager.
func NewWorktreeManager(baseDir string) *WorktreeManager {
	// Find the repository root
	cmd := exec.Command("git", "rev-parse", "--show-toplevel")
	out, err := cmd.CombinedOutput()
	repoDir := ""
	if err == nil && len(out) > 0 {
		repoDir = string(out[:len(out)-1]) // trim newline
	} else {
		// Fallback to current directory if not in a git repo (e.g. some test environments)
		repoDir, _ = os.Getwd()
	}

	return &WorktreeManager{
		baseDir: baseDir,
		repoDir: repoDir,
	}
}

// Create provisions a new git worktree for the given task ID.
func (wm *WorktreeManager) Create(ctx context.Context, taskID string) (string, error) {
	if err := os.MkdirAll(wm.baseDir, 0755); err != nil {
		return "", fmt.Errorf("failed to create base worktree dir: %w", err)
	}

	worktreePath := filepath.Join(wm.baseDir, taskID)
	branchName := fmt.Sprintf("task-%s", taskID)

	cmd := exec.CommandContext(ctx, "git", "worktree", "add", worktreePath, "-b", branchName)
	cmd.Dir = wm.repoDir
	if out, err := cmd.CombinedOutput(); err != nil {
		return "", fmt.Errorf("git worktree add failed: %w, output: %s", err, string(out))
	}

	return worktreePath, nil
}

// Cleanup removes the git worktree for the given task ID.
func (wm *WorktreeManager) Cleanup(ctx context.Context, taskID string) error {
	worktreePath := filepath.Join(wm.baseDir, taskID)

	cmd := exec.CommandContext(ctx, "git", "worktree", "remove", "-f", worktreePath)
	cmd.Dir = wm.repoDir
	if out, err := cmd.CombinedOutput(); err != nil {
		os.RemoveAll(worktreePath)
		branchName := fmt.Sprintf("task-%s", taskID)
		bCmd := exec.CommandContext(ctx, "git", "branch", "-D", branchName)
		bCmd.Dir = wm.repoDir
		bCmd.Run()

		return fmt.Errorf("git worktree remove failed: %w, output: %s", err, string(out))
	}

	branchName := fmt.Sprintf("task-%s", taskID)
	bCmd := exec.CommandContext(ctx, "git", "branch", "-D", branchName)
	bCmd.Dir = wm.repoDir
	bCmd.Run()

	return nil
}
