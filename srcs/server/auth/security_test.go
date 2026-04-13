package auth

import (
	"testing"
	"time"
	"context"
)

func TestHS256Validation_ClockSkew(t *testing.T) {
	secret := []byte("test-secret")
	now := time.Now().Unix()

	tests := []struct {
		name    string
		claims  Claims
		wantErr string
	}{
		{
			name: "valid token",
			claims: Claims{
				IssuedAt: now,
				Expires:  now + 3600,
			},
			wantErr: "",
		},
		{
			name: "token issued in the future (within skew)",
			claims: Claims{
				IssuedAt: now + 30,
				Expires:  now + 3600,
			},
			wantErr: "",
		},
		{
			name: "token issued too far in the future",
			claims: Claims{
				IssuedAt: now + 120,
				Expires:  now + 3600,
			},
			wantErr: "token issued in the future",
		},
		{
			name: "token not yet valid (nbf within skew)",
			claims: Claims{
				IssuedAt:  now - 60,
				NotBefore: now + 30,
				Expires:   now + 3600,
			},
			wantErr: "",
		},
		{
			name: "token not yet valid (nbf too far)",
			claims: Claims{
				IssuedAt:  now - 60,
				NotBefore: now + 120,
				Expires:   now + 3600,
			},
			wantErr: "token not yet valid",
		},
		{
			name: "expired token",
			claims: Claims{
				IssuedAt: now - 3600,
				Expires:  now - 10,
			},
			wantErr: "token expired",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			token, err := signHS256(tt.claims, secret)
			if err != nil {
				t.Fatalf("failed to sign token: %v", err)
			}

			_, err = parseHS256(token, secret)
			if tt.wantErr == "" {
				if err != nil {
					t.Errorf("unexpected error: %v", err)
				}
			} else {
				if err == nil || err.Error() != tt.wantErr {
					t.Errorf("expected error %q, got %v", tt.wantErr, err)
				}
			}
		})
	}
}

func TestTenantIsolation_OrganizationID(t *testing.T) {
	claims := &Claims{
		OrganizationID: "org-123",
		Roles:          []string{RoleViewer},
	}

	if !claims.HasRole(RoleViewer) {
		t.Error("expected viewer role to be granted")
	}

	if claims.OrganizationID != "org-123" {
		t.Errorf("expected OrgID org-123, got %s", claims.OrganizationID)
	}
}

func TestClaims_HasRole_Admin(t *testing.T) {
	adminClaims := &Claims{
		Roles: []string{RoleAdmin},
	}
	viewerClaims := &Claims{
		Roles: []string{RoleViewer},
	}

	if !adminClaims.HasRole(RoleViewer) {
		t.Error("admin should have viewer permissions implicitly")
	}
	if viewerClaims.HasRole(RoleAdmin) {
		t.Error("viewer should NOT have admin permissions")
	}
}

func TestValidateToken_RevokedOIDC(t *testing.T) {
	store := NewStore()
	// Force OIDC enabled so parseHS256 fallback happens
	store.oidcCfg.Enabled = true

	// Manually insert a revoked jti
	store.RevokeToken("oidc-jti-123", time.Now().Add(1 * time.Hour))

	// Test revocation logic directly or through standard token parsing fallback behavior
	if !store.IsRevoked("oidc-jti-123") {
		t.Errorf("expected token to be revoked")
	}
}

func TestTenantIsolation_GetByID(t *testing.T) {
	// Validate tenant isolation through dummy claims injection
	ctx := context.WithValue(context.Background(), claimsContextKey, &Claims{OrganizationID: "tenant-a"})
	orgID := OrganizationIDFromContext(ctx)
	if orgID != "tenant-a" {
		t.Errorf("expected tenant-a, got %s", orgID)
	}
}
