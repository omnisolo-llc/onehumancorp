package agent

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/agent/harness"
	"github.com/onehumancorp/mono/srcs/server/harness/authz"
)

type Executor struct {
	harness     harness.IsolationHarness
	interceptor *authz.CapabilityInterceptor
}

func NewExecutor(h harness.IsolationHarness, interceptor *authz.CapabilityInterceptor) *Executor {
	return &Executor{harness: h, interceptor: interceptor}
}

func (e *Executor) ExecuteCommand(ctx context.Context, sessionID string, cmd string) ([]byte, error) {
	// If interceptor is not nil, use it; otherwise just execute
	if e.interceptor != nil {
		var result []byte
		err := e.interceptor.Intercept(ctx, sessionID, "bash", func() error {
			res, execErr := e.harness.Execute(ctx, harness.ExecutionContext{
				Command: []string{"/bin/sh", "-c", cmd},
			})
			result = res
			return execErr
		})
		return result, err
	}

	return e.harness.Execute(ctx, harness.ExecutionContext{
		Command: []string{"/bin/sh", "-c", cmd},
	})
}
