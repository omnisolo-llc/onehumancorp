package terminal


import (
	"context"

)

type Executor struct {
	harness   IsolationHarness
	validator CommandValidator
}

func NewExecutor(h IsolationHarness) *Executor {
	return &Executor{
		harness:   h,
		validator: NewDefaultCommandValidator(),
	}
}

func NewExecutorWithValidator(h IsolationHarness, v CommandValidator) *Executor {
	return &Executor{
		harness:   h,
		validator: v,
	}
}

func (e *Executor) ExecuteCommand(ctx context.Context, cmd string) ([]byte, error) {
	if err := e.validator.Validate(cmd); err != nil {
		return nil, err
	}


	// Get AgentContext to retrieve the proxy configuration
	var networkProxy string

	return e.harness.Execute(ctx, ExecutionContext{
		Command: []string{"/bin/sh", "-c", cmd},
		NetworkProxy: networkProxy,
	})
}

type IsolationHarness interface {
	Execute(ctx context.Context, ec ExecutionContext) ([]byte, error)
}
type ExecutionContext struct {
	Command []string
	NetworkProxy string
}
