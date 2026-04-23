package terminal

import (
	"context"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/harness"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type Executor struct {
	harness      harness.IsolationHarness
	validator    CommandValidator
	policyEngine *harness.PolicyEngine
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

func (e *Executor) WithPolicyEngine(pe *harness.PolicyEngine) *Executor {
	e.policyEngine = pe
	return e
}

func (e *Executor) ExecuteCommand(ctx context.Context, cmd string) ([]byte, error) {
	if err := e.validator.Validate(cmd); err != nil {
		return nil, err
	}

	if e.policyEngine != nil {
		allowed, err := e.policyEngine.CheckPolicy(ctx, cmd)
		if err != nil {
			return nil, fmt.Errorf("policy check failed: %w", err)
		}
		if !allowed {
			return nil, fmt.Errorf("command execution denied by policy")
		}
	}

	// Get AgentContext to retrieve the proxy configuration
	var networkProxy string
	if ac, ok := orchestration.GetAgentContext(ctx); ok {
		if proxy, exists := ac.Env["HTTP_PROXY"]; exists {
			networkProxy = proxy
		}
	}

	return e.harness.Execute(ctx, harness.ExecutionContext{
		Command:      []string{"/bin/sh", "-c", cmd},
		NetworkProxy: networkProxy,
	})
}
