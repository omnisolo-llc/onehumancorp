package authz

import (
	"context"
	"encoding/json"
	"errors"
	"testing"
)

func TestCapabilityInterceptor_Intercept(t *testing.T) {
	data, _ := json.Marshal([]string{"read", "write"})
	provider := &mockProvider{
		row: &mockRow{
			data: data,
		},
	}
	authorizer := NewCapabilityAuthorizer(provider)
	interceptor := NewCapabilityInterceptor(authorizer)

	t.Run("Allowed", func(t *testing.T) {
		executed := false
		err := interceptor.Intercept(context.Background(), "session1", "read", func() error {
			executed = true
			return nil
		})

		if err != nil {
			t.Errorf("Unexpected error: %v", err)
		}
		if !executed {
			t.Error("Expected execute function to be called")
		}
	})

	t.Run("Denied", func(t *testing.T) {
		executed := false
		err := interceptor.Intercept(context.Background(), "session1", "admin", func() error {
			executed = true
			return nil
		})

		if !errors.Is(err, ErrCapabilityDenied) {
			t.Errorf("Expected ErrCapabilityDenied, got %v", err)
		}
		if executed {
			t.Error("Expected execute function NOT to be called")
		}
	})
}
