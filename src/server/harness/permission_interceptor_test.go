package harness

import (
	"context"
	"errors"
	"io"
	"testing"
)

type mockPolicyExecutor struct {
	called bool
}

func (m *mockPolicyExecutor) Execute(ctx context.Context, command string) (Result, error) {
	m.called = true
	return Result{Stdout: "executed"}, nil
}

func (m *mockPolicyExecutor) ExecuteWithPolicy(ctx context.Context, command string, policy *Policy) (Result, error) {
	m.called = true
	return Result{Stdout: "executed_policy"}, nil
}

func (m *mockPolicyExecutor) ExecuteStream(ctx context.Context, command string, policy *Policy, stdin io.Reader, stdout, stderr io.Writer) (Result, error) {
	m.called = true
	return Result{Stdout: "executed_stream"}, nil
}

type mockBridge struct {
	authResp *AuthorizationResponse
	err      error
}

func (m *mockBridge) RequestPermission(ctx context.Context, req PermissionRequest) (*AuthorizationResponse, error) {
	return m.authResp, m.err
}

func TestPermissionInterceptor_Execute(t *testing.T) {
	target := &mockPolicyExecutor{}

	t.Run("authorized execution", func(t *testing.T) {
		bridge := &mockBridge{
			authResp: &AuthorizationResponse{Authorized: true},
		}
		interceptor := NewPermissionInterceptor(target, bridge)

		target.called = false
		res, err := interceptor.Execute(context.Background(), "echo test")

		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if res.Stdout != "executed" {
			t.Errorf("expected executed, got %s", res.Stdout)
		}
		if !target.called {
			t.Errorf("expected target to be called")
		}
	})

	t.Run("denied execution", func(t *testing.T) {
		bridge := &mockBridge{
			authResp: &AuthorizationResponse{Authorized: false, Reason: "unauthorized tool"},
		}
		interceptor := NewPermissionInterceptor(target, bridge)

		target.called = false
		_, err := interceptor.Execute(context.Background(), "echo test")

		if err == nil {
			t.Fatalf("expected error, got nil")
		}
		if target.called {
			t.Errorf("expected target to not be called")
		}
	})

	t.Run("bridge error", func(t *testing.T) {
		bridge := &mockBridge{
			err: errors.New("network error"),
		}
		interceptor := NewPermissionInterceptor(target, bridge)

		target.called = false
		_, err := interceptor.Execute(context.Background(), "echo test")

		if err == nil {
			t.Fatalf("expected error, got nil")
		}
		if target.called {
			t.Errorf("expected target to not be called")
		}
	})
}
