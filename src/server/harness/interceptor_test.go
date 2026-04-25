package harness

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/src/server/harness/authz"
)

type mockAuthorizer struct {
	err error
}

func (m *mockAuthorizer) Authorize(ctx context.Context, sessionID string, capability string) error {
	return m.err
}

type mockTargetHarness struct {
	called bool
}

func (m *mockTargetHarness) Execute(ctx context.Context, command string) (Result, error) {
	m.called = true
	return Result{Stdout: "success"}, nil
}

func TestAuthorizingHarness_Execute(t *testing.T) {
	target := &mockTargetHarness{}

	t.Run("success without authorizer", func(t *testing.T) {
		h := NewAuthorizingHarness(target, nil)
		_, err := h.Execute(context.Background(), "bash")
		if err != nil {
			t.Errorf("unexpected error: %v", err)
		}
	})

	t.Run("capability denied", func(t *testing.T) {
		auth := &mockAuthorizer{err: authz.ErrCapabilityDenied}
		h := NewAuthorizingHarness(target, auth)

		ctx := context.WithValue(context.Background(), SessionContextKey{}, "sess1")
		_, err := h.Execute(ctx, "bash")
		if err != authz.ErrCapabilityDenied {
			t.Errorf("expected ErrCapabilityDenied, got %v", err)
		}
	})

	t.Run("capability allowed", func(t *testing.T) {
		auth := &mockAuthorizer{err: nil}
		h := NewAuthorizingHarness(target, auth)

		ctx := context.WithValue(context.Background(), SessionContextKey{}, "sess1")
		res, err := h.Execute(ctx, "bash")
		if err != nil {
			t.Errorf("unexpected error: %v", err)
		}
		if res.Stdout != "success" {
			t.Errorf("expected success, got %s", res.Stdout)
		}
	})
}
