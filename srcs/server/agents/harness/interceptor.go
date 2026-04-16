package harness

import (
	"context"
	"fmt"
)

// ExecutionPolicyInterceptor checks the policy engine before allowing command execution.
type ExecutionPolicyInterceptor struct {
	Engine *PolicyEngine
}

// NewExecutionPolicyInterceptor creates a new interceptor.
func NewExecutionPolicyInterceptor(engine *PolicyEngine) *ExecutionPolicyInterceptor {
	return &ExecutionPolicyInterceptor{
		Engine: engine,
	}
}

// Intercept validates the command against the policy engine.
// Returns an error if the command is denied.
func (i *ExecutionPolicyInterceptor) Intercept(ctx context.Context, command string) error {
	if i.Engine == nil {
		return nil // Fail-open or close? Let's fail-open if not configured to avoid breaking existing agents, or maybe we shouldn't.
	}

	if !i.Engine.CheckPolicy(ctx, command) {
		return fmt.Errorf("execution denied by policy: %s", command)
	}

	return nil
}

// Global default engine for easy integration, or should it be injected?
var GlobalPolicyEngine = NewPolicyEngine()
var GlobalInterceptor = NewExecutionPolicyInterceptor(GlobalPolicyEngine)

func init() {
    // Load some default policies for the global engine
    GlobalPolicyEngine.LoadPoliciesFromDB()
}
