package harness

import (
	"context"
)

// HarnessBackend defines the flexible execution interface for OHC agents.
type HarnessBackend interface {
	Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error)
}

type BackendType string

const (
	BackendTypeLocal      BackendType = "local"
	BackendTypeDocker     BackendType = "docker"
	BackendTypeServerless BackendType = "serverless"
)

// HarnessGateway routes agent tasks to the correct execution backend.
type HarnessGateway struct {
	backends map[BackendType]HarnessBackend
}

// NewHarnessGateway creates a new registry of execution backends.
func NewHarnessGateway() *HarnessGateway {
	gateway := &HarnessGateway{
		backends: make(map[BackendType]HarnessBackend),
	}
	// Initialize with our implemented backends
	gateway.RegisterBackend(BackendTypeLocal, NewLocalBackend())
	gateway.RegisterBackend(BackendTypeDocker, NewDockerBackend())
	gateway.RegisterBackend(BackendTypeServerless, NewServerlessBackend())
	return gateway
}

// RegisterBackend registers an implementation for a given backend type.
func (g *HarnessGateway) RegisterBackend(bType BackendType, backend HarnessBackend) {
	g.backends[bType] = backend
}

// Execute routes the execution to the appropriate backend based on tier.
func (g *HarnessGateway) Execute(ctx context.Context, execCtx ExecutionContext, tier string) ([]byte, error) {
	bType := resolveBackendForTier(tier)
	backend, exists := g.backends[bType]
	if !exists {
		// Fallback to local if requested backend type doesn't exist
		backend = g.backends[BackendTypeLocal]
	}
	return backend.Execute(ctx, execCtx)
}

func resolveBackendForTier(tier string) BackendType {
	switch tier {
	case "free":
		return BackendTypeServerless
	case "standard":
		return BackendTypeDocker
	case "premium":
		return BackendTypeLocal
	default:
		// Default to serverless for idle/unknown to lower cost
		return BackendTypeServerless
	}
}

// LocalBackend wraps the current bwrap execution.
type LocalBackend struct {
	bwrap *BwrapHarness
}

func NewLocalBackend() *LocalBackend {
	return &LocalBackend{
		bwrap: NewBwrapHarness(),
	}
}

func (l *LocalBackend) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	return l.bwrap.Execute(ctx, execCtx)
}
