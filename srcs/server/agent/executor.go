package agent

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/agent/harness"
)

type Executor struct {
	harness harness.IsolationHarness
}

func NewExecutor(h harness.IsolationHarness) *Executor {
	return &Executor{harness: h}
}

func (e *Executor) ExecuteCommand(ctx context.Context, cmd string) ([]byte, error) {
	return e.harness.Execute(ctx, harness.ExecutionContext{
		Command: []string{"/bin/sh", "-c", cmd},
	})
}
