package auth

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"strings"
	"crypto/ed25519"
)

func TestHandlePowerSyncJWKS(t *testing.T) {
	h := NewHandlers(nil)
	req := httptest.NewRequest("GET", "/api/auth/powersync/jwks", nil)
	rec := httptest.NewRecorder()

	h.HandlePowerSyncJWKS(rec, req)

	if rec.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", rec.Code)
	}

	var jwks map[string]interface{}
	if err := json.Unmarshal(rec.Body.Bytes(), &jwks); err != nil {
		t.Fatalf("failed to parse JSON: %v", err)
	}

	keys, ok := jwks["keys"].([]interface{})
	if !ok || len(keys) != 1 {
		t.Fatalf("expected 1 key in jwks, got %v", jwks["keys"])
	}

	key := keys[0].(map[string]interface{})
	if key["kty"] != "OKP" || key["crv"] != "Ed25519" || key["kid"] != "powersync-key-1" {
		t.Errorf("unexpected key properties: %v", key)
	}
}

func TestHandlePowerSyncToken(t *testing.T) {
	h := NewHandlers(nil)

	// Test without claims
	req := httptest.NewRequest("GET", "/api/auth/powersync/token", nil)
	rec := httptest.NewRecorder()
	h.HandlePowerSyncToken(rec, req)
	if rec.Code != http.StatusUnauthorized {
		t.Errorf("expected 401, got %d", rec.Code)
	}

	// Test with claims
	claims := &Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	}
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, claims)
	req = httptest.NewRequest("GET", "/api/auth/powersync/token", nil).WithContext(ctx)
	rec = httptest.NewRecorder()

	h.HandlePowerSyncToken(rec, req)

	if rec.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", rec.Code)
	}

	var resp map[string]string
	if err := json.Unmarshal(rec.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to parse JSON: %v", err)
	}

	tokenString, ok := resp["token"]
	if !ok || tokenString == "" {
		t.Fatal("expected token in response")
	}

	parts := strings.Split(tokenString, ".")
	if len(parts) != 3 {
		t.Fatalf("expected 3 token parts, got %d", len(parts))
	}

	payBytes, _ := b64urlDecode(parts[1])
	var mapClaims map[string]interface{}
	json.Unmarshal(payBytes, &mapClaims)

	if mapClaims["sub"] != "user-1" {
		t.Errorf("expected sub user-1, got %v", mapClaims["sub"])
	}

	params := mapClaims["parameters"].(map[string]interface{})
	if params["organization_id"] != "org-1" {
		t.Errorf("expected organization_id org-1, got %v", params["organization_id"])
	}

	// Verify signature
	sigInput := parts[0] + "." + parts[1]
	sig, _ := b64urlDecode(parts[2])
	if !ed25519.Verify(powerSyncPublicKey, []byte(sigInput), sig) {
		t.Error("signature verification failed")
	}
}

func TestHandlePowerSyncToken_DefaultOrg(t *testing.T) {
	h := NewHandlers(nil)

	claims := &Claims{
		Subject:        "user-1",
		OrganizationID: "",
		Roles:          []string{"system"},
	}
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, claims)
	req := httptest.NewRequest("GET", "/api/auth/powersync/token", nil).WithContext(ctx)
	rec := httptest.NewRecorder()

	h.HandlePowerSyncToken(rec, req)

	if rec.Code != http.StatusOK {
		t.Errorf("expected status 200, got %d", rec.Code)
	}

	var resp map[string]string
	json.Unmarshal(rec.Body.Bytes(), &resp)

	parts := strings.Split(resp["token"], ".")
	payBytes, _ := b64urlDecode(parts[1])
	var mapClaims map[string]interface{}
	json.Unmarshal(payBytes, &mapClaims)

	params := mapClaims["parameters"].(map[string]interface{})
	if params["organization_id"] != "default_org" {
		t.Errorf("expected default_org, got %v", params["organization_id"])
	}
}
