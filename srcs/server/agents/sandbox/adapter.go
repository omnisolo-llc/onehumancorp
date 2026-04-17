package sandbox

import (
	"context"
)

// Config holds the configuration for the sandbox execution,
// particularly the volume bindings required for isolation.
type Config struct {
	// Bind maps host paths to virtual sandbox paths (read-write).
	Bind map[string]string
	// RoBind maps host paths to virtual sandbox paths (read-only).
	RoBind map[string]string
}

// Result holds the standard output and standard error from the execution.
type Result struct {
	Stdout string
	Stderr string
}

// HarnessAdapter defines the interface for safely executing shell commands.
type HarnessAdapter interface {
	Execute(ctx context.Context, cmd string, cfg Config) (*Result, error)
}
