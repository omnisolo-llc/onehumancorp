package sandbox

import (
	"os"
	"path/filepath"
	"testing"
)

func TestGitScrubber(t *testing.T) {
	// Create a temporary directory to act as the working directory.
	tempDir, err := os.MkdirTemp("", "git-scrubber-test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	// Pre-create a legitimate 'config' file to simulate a legitimate git repo artifact.
	configPath := filepath.Join(tempDir, "config")
	if err := os.WriteFile(configPath, []byte("repo config"), 0644); err != nil {
		t.Fatalf("Failed to create config file: %v", err)
	}

	scrubber := NewGitScrubber(tempDir)

	// Step 1: PreCommand
	if err := scrubber.PreCommand(); err != nil {
		t.Fatalf("PreCommand failed: %v", err)
	}

	// Step 2: Simulate malicious agent planting git artifacts.
	maliciousHEADPath := filepath.Join(tempDir, "HEAD")
	if err := os.WriteFile(maliciousHEADPath, []byte("ref: refs/heads/master"), 0644); err != nil {
		t.Fatalf("Failed to create malicious HEAD file: %v", err)
	}

	maliciousHooksPath := filepath.Join(tempDir, "hooks")
	if err := os.Mkdir(maliciousHooksPath, 0755); err != nil {
		t.Fatalf("Failed to create malicious hooks dir: %v", err)
	}
	// Add a dummy hook file to ensure directory removal works correctly.
	if err := os.WriteFile(filepath.Join(maliciousHooksPath, "pre-commit"), []byte("#!/bin/sh\necho hook"), 0755); err != nil {
		t.Fatalf("Failed to create malicious hook file: %v", err)
	}

	// Step 3: PostCommand
	if err := scrubber.PostCommand(); err != nil {
		t.Fatalf("PostCommand failed: %v", err)
	}

	// Verify the pre-existing legitimate file is still there.
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		t.Errorf("Legitimate file 'config' was incorrectly removed")
	}

	// Verify the maliciously planted file and directory are removed.
	if _, err := os.Stat(maliciousHEADPath); !os.IsNotExist(err) {
		t.Errorf("Maliciously planted 'HEAD' file was not removed")
	}

	if _, err := os.Stat(maliciousHooksPath); !os.IsNotExist(err) {
		t.Errorf("Maliciously planted 'hooks' directory was not removed")
	}
}
