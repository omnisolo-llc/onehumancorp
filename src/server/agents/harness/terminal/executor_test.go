package terminal

import (
	"context"
	"fmt"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/src/server/agents/harness"
	"github.com/onehumancorp/mono/src/server/db"
)

type mockRows struct {
	action string
	called bool
}

func (m *mockRows) Next() bool {
	if !m.called {
		m.called = true
		return true
	}
	return false
}
func (m *mockRows) Scan(dest ...any) error {
	if len(dest) > 0 {
		if ptr, ok := dest[0].(*string); ok {
			*ptr = m.action
		}
	}
	return nil
}
func (m *mockRows) Close()                     {}
func (m *mockRows) Err() error                 { return nil }
func (m *mockRows) Columns() ([]string, error) { return []string{"action"}, nil }

type mockProvider struct {
	db.Provider
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	cmd := optionsAndArgs[0].(string)
	if strings.Contains(cmd, "cat /etc/shadow") {
		return &mockRows{action: "ALLOW"}, nil
	}
	if strings.Contains(cmd, "echo 'test'") {
		return &mockRows{action: "ALLOW"}, nil
	}
	if strings.Contains(cmd, "zmodload") {
		return &mockRows{action: "ALLOW"}, nil
	}
	if strings.Contains(cmd, "deny_me") {
		return &mockRows{action: "DENY"}, nil
	}
	if strings.Contains(cmd, "fail_engine") {
		return nil, fmt.Errorf("engine failure mock")
	}
	return nil, fmt.Errorf("no such table")
}

func TestExecutor_E2E_ShadowAccess(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	pe := harness.NewPolicyEngine(&mockProvider{})
	exec := NewExecutor(realHarness).WithPolicyEngine(pe)

	out, err := exec.ExecuteCommand(context.Background(), "cat /etc/shadow")

	if err == nil {
		t.Fatalf("expected an error or failure when accessing /etc/shadow, got nil. Output: %s", string(out))
	}

	outStr := string(out)
	errStr := err.Error()

	if !strings.Contains(outStr, "Permission denied") &&
		!strings.Contains(outStr, "No such file") &&
		!strings.Contains(outStr, "not permitted") &&
		!strings.Contains(errStr, "not found") &&
		!strings.Contains(errStr, "exit status") {
		t.Fatalf("Unexpected output when trying to access /etc/shadow: out=%s, err=%s", outStr, errStr)
	}
}

func TestExecutor_Success(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	pe := harness.NewPolicyEngine(&mockProvider{})
	exec := NewExecutor(realHarness).WithPolicyEngine(pe)

	out, err := exec.ExecuteCommand(context.Background(), "echo 'test'")

	if err != nil {
		if strings.Contains(err.Error(), "not found") {
			t.Skipf("Skipping success test because bwrap/sandbox-exec is not installed: %v", err)
			return
		}
	}

	if err == nil && !strings.Contains(string(out), "test") {
		t.Fatalf("expected output to contain 'test', got: %s", string(out))
	}
}

func TestExecutor_ValidationFailure(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	pe := harness.NewPolicyEngine(&mockProvider{})
	exec := NewExecutor(realHarness).WithPolicyEngine(pe)

	out, err := exec.ExecuteCommand(context.Background(), "zmodload zsh/net/tcp")

	if err == nil {
		t.Fatalf("expected validation error, got nil. Output: %s", string(out))
	}

	if err != ErrDangerousZSHBuiltin {
		t.Fatalf("expected ErrDangerousZSHBuiltin, got: %v", err)
	}
}

func TestExecutor_PolicyDenied(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	pe := harness.NewPolicyEngine(&mockProvider{})
	exec := NewExecutor(realHarness).WithPolicyEngine(pe)

	_, err := exec.ExecuteCommand(context.Background(), "deny_me")
	if err == nil || !strings.Contains(err.Error(), "denied by policy") {
		t.Fatalf("expected policy denied error, got: %v", err)
	}
}

func TestExecutor_PolicyEngineFailure(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	pe := harness.NewPolicyEngine(&mockProvider{})
	exec := NewExecutor(realHarness).WithPolicyEngine(pe)

	_, err := exec.ExecuteCommand(context.Background(), "fail_engine")
	if err == nil || !strings.Contains(err.Error(), "policy check failed") {
		t.Fatalf("expected policy check failed error, got: %v", err)
	}
}

func TestExecutor_NewExecutorWithValidator(t *testing.T) {
	realHarness := harness.NewIsolationHarness()
	pe := harness.NewPolicyEngine(&mockProvider{})
	validator := NewDefaultCommandValidator()
	exec := NewExecutorWithValidator(realHarness, validator).WithPolicyEngine(pe)

	_, _ = exec.ExecuteCommand(context.Background(), "echo 'test'")
}
