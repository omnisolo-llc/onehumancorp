package terminal

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/agents/harness"
)

type Executor struct {
	harness   harness.IsolationHarness
	validator CommandValidator
}

func NewExecutor(h harness.IsolationHarness) *Executor {
	return &Executor{
		harness:   h,
		validator: NewDefaultCommandValidator(),
	}
}

func NewExecutorWithValidator(h harness.IsolationHarness, v CommandValidator) *Executor {
	return &Executor{
		harness:   h,
		validator: v,
	}
}

func (e *Executor) ExecuteCommand(ctx context.Context, cmd string) ([]byte, error) {
	if err := e.validator.Validate(cmd); err != nil {
		return nil, err
	}

	return e.harness.Execute(ctx, harness.ExecutionContext{
		Command: []string{"/bin/sh", "-c", cmd},
	})
}
