package harness

import "context"

type ExecutionResult struct {
	Stdout   string
	Stderr   string
	ExitCode int
}

type SandboxBackend interface {
	ExecuteCommand(ctx context.Context, cmd string) (*ExecutionResult, error)
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
}
