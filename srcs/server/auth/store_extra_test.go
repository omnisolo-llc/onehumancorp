package auth

import (
	"testing"
)

func TestGetOrCreateOIDCUser_TenantIsolation(t *testing.T) {
	store := NewStore()

	// Create an OIDC user for org A
	u1 := store.GetOrCreateOIDCUser("sub-123", "test@test.com", "testuser", "org-A")

	// Create an OIDC user for org B with same sub and email
	u2 := store.GetOrCreateOIDCUser("sub-123", "test@test.com", "testuser", "org-B")

	if u1.ID == u2.ID {
		t.Errorf("expected different users for different orgs, got same ID: %s", u1.ID)
	}
	if u1.OrganizationID != "org-A" {
		t.Errorf("expected u1 to belong to org-A, got %s", u1.OrganizationID)
	}
	if u2.OrganizationID != "org-B" {
		t.Errorf("expected u2 to belong to org-B, got %s", u2.OrganizationID)
	}

	// Fetch again for org A, should get u1
	u1_fetch := store.GetOrCreateOIDCUser("sub-123", "test@test.com", "testuser", "org-A")
	if u1_fetch.ID != u1.ID {
		t.Errorf("expected to fetch same user for org-A")
	}
}
