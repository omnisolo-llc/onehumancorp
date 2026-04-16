package harness

import (
	"context"
	"testing"
)

func TestExecutionPolicyInterceptor_Intercept(t *testing.T) {
	engine := NewPolicyEngine()
	engine.AddAllowRule("ls")
	engine.AddDenyRule("rm -rf")

	interceptor := NewExecutionPolicyInterceptor(engine)
	ctx := context.Background()

	err := interceptor.Intercept(ctx, "ls -la")
	if err != nil {
		t.Errorf("Expected nil error for allowed command, got: %v", err)
	}

	err = interceptor.Intercept(ctx, "rm -rf /")
	if err == nil {
		t.Errorf("Expected error for denied command, got nil")
	}
}
