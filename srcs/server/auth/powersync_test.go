package auth_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"strings"

	"github.com/onehumancorp/ohc-api/srcs/server/auth"
)

func TestPowerSyncTokenGeneration(t *testing.T) {
	keypair, err := auth.GeneratePowerSyncKeypair()
	if err != nil {
		t.Fatalf("Failed to generate keypair: %v", err)
	}

	claims := &auth.Claims{
		Subject: "user-1",
		OrganizationID: "org-1",
	}

	token, err := auth.IssuePowerSyncToken(claims, keypair)
	if err != nil {
		t.Fatalf("Failed to issue token: %v", err)
	}

	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		t.Fatalf("Expected 3 token parts, got %d", len(parts))
	}
}

func TestPowerSyncHandlers(t *testing.T) {
	keypair, err := auth.GeneratePowerSyncKeypair()
	if err != nil {
		t.Fatalf("Failed to generate keypair: %v", err)
	}

	// Test JWKS handler
	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	w := httptest.NewRecorder()

	auth.PowerSyncJWKSHandler(keypair).ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("Expected status 200, got %d", w.Code)
	}

	var jwks map[string]interface{}
	if err := json.Unmarshal(w.Body.Bytes(), &jwks); err != nil {
		t.Fatalf("Failed to unmarshal JWKS: %v", err)
	}

	// Test Token handler
	reqToken := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)

	claims := &auth.Claims{
		Subject: "user-1",
		OrganizationID: "org-1",
	}

	ctx := context.WithValue(reqToken.Context(), auth.ClaimsContextKeyForTest, claims)
	reqToken = reqToken.WithContext(ctx)

	wToken := httptest.NewRecorder()
	auth.PowerSyncTokenHandler(nil, keypair).ServeHTTP(wToken, reqToken)

	if wToken.Code != http.StatusOK {
		t.Fatalf("Expected status 200, got %d", wToken.Code)
	}
}
