package harness

import "context"

type ExecutionContext struct {
	Command      []string
	AllowedPaths []string
	NetworkProxy string
}

type IsolationHarness interface {
	Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error)
}
