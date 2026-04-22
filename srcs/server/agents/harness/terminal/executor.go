package terminal

import (
	"fmt"
	"context"

	"github.com/onehumancorp/mono/srcs/server/agents/harness"
	serverharness "github.com/onehumancorp/mono/srcs/server/harness"
	"github.com/onehumancorp/mono/srcs/server/harness/sandbox"
)

type Executor struct {
	harness   harness.IsolationHarness
	validator CommandValidator
	sm        *sandbox.SandboxManager
}

func NewExecutor(h harness.IsolationHarness) *Executor {
	return &Executor{
		sm:        sandbox.NewSandboxManager(serverharness.Config{}, nil),
		harness:   h,
		validator: NewDefaultCommandValidator(),
	}
}

func NewExecutorWithValidator(h harness.IsolationHarness, v CommandValidator) *Executor {
	return &Executor{
		sm:        sandbox.NewSandboxManager(serverharness.Config{}, nil),
		harness:   h,
		validator: v,
	}
}

func (e *Executor) ExecuteCommand(ctx context.Context, cmd string) ([]byte, error) {
	if err := e.validator.Validate(cmd); err != nil {
		return nil, err
	}

	wrappedCmd, err := e.sm.WrapCommand(ctx, cmd)
	if err != nil {
		return nil, err
	}

	res, err := e.harness.Execute(ctx, harness.ExecutionContext{
		Command: []string{"/bin/sh", "-c", wrappedCmd},
	})

	if err != nil {
		return nil, fmt.Errorf("%s", e.sm.AnnotateError(err, string(res)))
	}

	return res, err
}
