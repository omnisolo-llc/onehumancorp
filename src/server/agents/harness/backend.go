package harness

import "context"

type HarnessBackend interface {
	Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error)
}

type LocalBackend struct {
	Isolation IsolationHarness
}

func (l *LocalBackend) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	isolation := l.Isolation
	if isolation == nil {
		isolation = NewIsolationHarness()
	}
	return isolation.Execute(ctx, execCtx)
}

type DockerBackend struct{}

func (d *DockerBackend) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	// Mock implementation for spinning up a container per agent session
	return []byte("executed in docker"), nil
}
