package sandbox

import (
	"os"
	"path/filepath"
)

type GitScrubber struct {
	bareGitRepoScrubPaths []string
	workingDirectory      string
}

func NewGitScrubber(workingDirectory string) *GitScrubber {
	return &GitScrubber{
		workingDirectory: workingDirectory,
	}
}

// PreCommand checks for the presence of git artifacts and records those that
// do not exist so they can be scrubbed post-execution.
func (s *GitScrubber) PreCommand() error {
	gitArtifacts := []string{"HEAD", "objects", "refs", "hooks", "config", ".git"}
	s.bareGitRepoScrubPaths = nil

	for _, artifact := range gitArtifacts {
		path := filepath.Join(s.workingDirectory, artifact)
		if _, err := os.Lstat(path); os.IsNotExist(err) {
			s.bareGitRepoScrubPaths = append(s.bareGitRepoScrubPaths, path)
		}
	}
	return nil
}

// PostCommand forcefully removes any git artifacts that were created during execution.
func (s *GitScrubber) PostCommand() error {
	for _, path := range s.bareGitRepoScrubPaths {
		os.RemoveAll(path) // ignoring error to make sure we scrub all possible entries
	}
	s.bareGitRepoScrubPaths = nil
	return nil
}
