package terminal


import (
	"context"

	"github.com/onehumancorp/mono/src/server_old/agents/harness"
	"github.com/onehumancorp/mono/src/server/orchestration"
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


	// Get AgentContext to retrieve the proxy configuration
	var networkProxy string
	if ac, ok := orchestration.GetAgentContext(ctx); ok {
		if proxy, exists := ac.Env["HTTP_PROXY"]; exists {
			networkProxy = proxy
		}
	}

	return e.harness.Execute(ctx, harness.ExecutionContext{
		Command: []string{"/bin/sh", "-c", cmd},
		NetworkProxy: networkProxy,
	})
}
