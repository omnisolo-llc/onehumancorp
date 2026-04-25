package sandbox

import (
	"context"
	"os"
	"path/filepath"

	"github.com/onehumancorp/mono/src/server/harness"
)

var GitSensitiveFiles = []string{
	"HEAD",
	"objects",
	"refs",
	"hooks",
	"config",
}

// GitScrubberInterceptor wraps a harness.AgentHarness to provide pre/post execution scrubbing
type GitScrubberInterceptor struct {
	next    harness.AgentHarness
	workDir string
}

// NewGitScrubberInterceptor creates a new interceptor with the given working directory.
func NewGitScrubberInterceptor(next harness.AgentHarness, workDir string) *GitScrubberInterceptor {
	return &GitScrubberInterceptor{
		next:    next,
		workDir: workDir,
	}
}

// Execute performs the pre-command checks, delegates execution to the wrapped harness, and scrubs maliciously created git files.
func (g *GitScrubberInterceptor) Execute(ctx context.Context, command string) (harness.Result, error) {
	// Pre-Command: record which git sensitive files do NOT exist
	var bareGitRepoScrubPaths []string

	for _, item := range GitSensitiveFiles {
		// Check standard .git/ location
		gitDirPath := filepath.Join(g.workDir, ".git", item)
		if _, err := os.Stat(gitDirPath); os.IsNotExist(err) {
			bareGitRepoScrubPaths = append(bareGitRepoScrubPaths, gitDirPath)
		}

		// Also check bare repo locations in the workDir directly
		bareDirPath := filepath.Join(g.workDir, item)
		if _, err := os.Stat(bareDirPath); os.IsNotExist(err) {
			bareGitRepoScrubPaths = append(bareGitRepoScrubPaths, bareDirPath)
		}
	}

	// Post-Command: scrub any paths that were created during execution.
	// We use defer to ensure cleanup happens even if next.Execute() panics.
	defer func() {
		for _, path := range bareGitRepoScrubPaths {
			_ = os.RemoveAll(path)
		}
	}()

	// Execute command via the underlying harness
	return g.next.Execute(ctx, command)
}
