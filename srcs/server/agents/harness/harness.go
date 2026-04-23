package harness

import "context"

type ExecutionContext struct {
	Command      []string
	AllowedPaths []string
	NetworkProxy string
}

type IsolationHarness interface {
	Execute(ctx context.Context, execCtx ExecutionContext) (stdout []byte, stderr []byte, err error)
}
