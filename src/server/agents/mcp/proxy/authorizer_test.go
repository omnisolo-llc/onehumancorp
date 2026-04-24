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

	// Test wildcard allowed capability
	profileWildcardAllow := CapabilityProfile{
		AllowedCapabilities: []string{"*"},
	}
	authorizer.SetProfile("session-wildcard-allow", profileWildcardAllow)
	err = authorizer.Authorize(ctx, "session-wildcard-allow", "anything", "any-tool")
	if err != nil {
		t.Errorf("Expected allowed capability for wildcard, got error: %v", err)
	}

	// Test wildcard denied capability
	profileWildcardDeny := CapabilityProfile{
		AllowedCapabilities: []string{"read"},
		DeniedCapabilities:  []string{"*"},
	}
	authorizer.SetProfile("session-wildcard-deny", profileWildcardDeny)
	err = authorizer.Authorize(ctx, "session-wildcard-deny", "read", "read-tool")
	if err == nil {
		t.Error("Expected error for explicitly denied wildcard capability, got nil")
	}

	// Test nil store initialization
	authorizerNilStore := NewCapabilityAuthorizer(nil)
	if authorizerNilStore.violationStore == nil {
		t.Error("Expected default violation store to be created, got nil")
	}
}
