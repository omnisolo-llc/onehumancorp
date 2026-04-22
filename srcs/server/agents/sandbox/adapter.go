package sandbox

import (
	"context"
)

// Config holds configuration for the harness adapter execution.
type Config struct {
	// Binds maps host paths to sandbox paths for read-write bind mounts.
	Binds map[string]string

	// RoBinds maps host paths to sandbox paths for read-only bind mounts.
	RoBinds map[string]string
}

// Result holds the result of a harness adapter execution.
type Result struct {
	// Output contains the combined standard output and standard error.
	Output string
}

// HarnessAdapter defines the interface for native process isolation layers.
type HarnessAdapter interface {
	// Execute runs a given shell command within the configured sandbox environment.
	Execute(ctx context.Context, cmd string, cfg Config) (*Result, error)
}
