package harness

import (
	"context"
	"testing"
)

func TestPolicyEngine_AllowSafe(t *testing.T) {
	pe := NewPolicyEngine()
	pe.AddAllowRule("ls")
	pe.AddAllowRule("echo")

	ctx := context.Background()

	if !pe.CheckPolicy(ctx, "ls -la") {
		t.Errorf("Expected 'ls -la' to be allowed")
	}

	if !pe.CheckPolicy(ctx, "echo 'hello'") {
		t.Errorf("Expected 'echo \\'hello\\'' to be allowed")
	}
}

func TestPolicyEngine_DenyMalicious(t *testing.T) {
	pe := NewPolicyEngine()
	pe.AddDenyRule("rm -rf")
	pe.AddDenyRule("sudo")

	ctx := context.Background()

	if pe.CheckPolicy(ctx, "rm -rf /") {
		t.Errorf("Expected 'rm -rf /' to be denied")
	}

	if pe.CheckPolicy(ctx, "sudo su") {
		t.Errorf("Expected 'sudo su' to be denied")
	}
}

func TestPolicyEngine_DenyChained(t *testing.T) {
    pe := NewPolicyEngine()
    pe.AddDenyRule("rm -rf")

    ctx := context.Background()

    if pe.CheckPolicy(ctx, "ls && rm -rf /") {
        t.Errorf("Expected 'ls && rm -rf /' to be denied")
    }

    if pe.CheckPolicy(ctx, "echo hello; rm -rf /") {
        t.Errorf("Expected 'echo hello; rm -rf /' to be denied")
    }
}
