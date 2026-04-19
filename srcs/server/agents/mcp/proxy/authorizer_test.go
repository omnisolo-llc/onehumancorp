package proxy

import (
	"context"
	"testing"
)

func TestCapabilityAuthorizer_Authorize(t *testing.T) {
	store := NewInMemoryViolationStore()
	authorizer := NewCapabilityAuthorizer(store)
	ctx := context.Background()

	sessionID := "session-1"

	// Test no profile (default deny)
	err := authorizer.Authorize(ctx, sessionID, "read", "fs-tool")
	if err == nil {
		t.Error("Expected error for no profile, got nil")
	}

	// Set profile
	profile := CapabilityProfile{
		AllowedCapabilities: []string{"read", "write"},
		DeniedCapabilities:  []string{"bash"},
	}
	authorizer.SetProfile(sessionID, profile)

	// Test allowed capability
	err = authorizer.Authorize(ctx, sessionID, "read", "fs-tool")
	if err != nil {
		t.Errorf("Expected allowed capability 'read', got error: %v", err)
	}

	// Test denied capability
	err = authorizer.Authorize(ctx, sessionID, "bash", "bash-tool")
	if err == nil {
		t.Error("Expected error for explicitly denied capability 'bash', got nil")
	}

	// Test implicitly denied capability
	err = authorizer.Authorize(ctx, sessionID, "execute", "exec-tool")
	if err == nil {
		t.Error("Expected error for implicitly denied capability 'execute', got nil")
	}

	// Check violation store
	violations, _ := store.GetViolations(ctx, sessionID)
	if len(violations) != 3 {
		t.Errorf("Expected 3 violations, got %d", len(violations))
	}
}
