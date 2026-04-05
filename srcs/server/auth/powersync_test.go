package auth

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestHandlePowerSyncJWKS(t *testing.T) {
	// Base64Url string for a 32 byte seed
	os.Setenv("OHC_POWERSYNC_PRIV_KEY", "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE")
	defer os.Unsetenv("OHC_POWERSYNC_PRIV_KEY")

	store := NewStore()
	h := NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	rr := httptest.NewRecorder()

	h.HandlePowerSyncJWKS(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var jwks JWKS
	if err := json.NewDecoder(rr.Body).Decode(&jwks); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(jwks.Keys) != 1 {
		t.Errorf("expected 1 key, got %d", len(jwks.Keys))
	}
	if len(jwks.Keys) > 0 {
		key := jwks.Keys[0]
		if key.Alg != "EdDSA" || key.Kty != "OKP" || key.Crv != "Ed25519" {
			t.Errorf("unexpected key properties: %+v", key)
		}
	}
}

func TestHandlePowerSyncToken(t *testing.T) {
	os.Setenv("OHC_POWERSYNC_PRIV_KEY", "MDEyMzQ1Njc4OTAxMjM0NTY3ODkwMTIzNDU2Nzg5MDE")
	defer os.Unsetenv("OHC_POWERSYNC_PRIV_KEY")

	store := NewStore()
	h := NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)

	// Inject claims into context
	claims := &Claims{
		Subject:        "user-123",
		OrganizationID: "org-1",
		Roles:          []string{"viewer"},
	}
	ctx := context.WithValue(req.Context(), claimsContextKey, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()

	h.HandlePowerSyncToken(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp PowerSyncTokenResponse
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Token == "" {
		t.Errorf("expected a token, got empty string")
	}
}

// Tests missing/invalid env var
func TestHandlePowerSyncConfigError(t *testing.T) {
	os.Unsetenv("OHC_POWERSYNC_PRIV_KEY")

	store := NewStore()
	h := NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	rr := httptest.NewRecorder()

	h.HandlePowerSyncJWKS(rr, req)
	if status := rr.Code; status != http.StatusInternalServerError {
		t.Errorf("handler returned wrong status code for missing key: got %v want %v", status, http.StatusInternalServerError)
	}

	os.Setenv("OHC_POWERSYNC_PRIV_KEY", "invalid_base64")
	req2 := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
	// Inject claims into context
	claims := &Claims{
		Subject:        "user-123",
		OrganizationID: "org-1",
		Roles:          []string{"viewer"},
	}
	ctx := context.WithValue(req2.Context(), claimsContextKey, claims)
	req2 = req2.WithContext(ctx)
	rr2 := httptest.NewRecorder()
	h.HandlePowerSyncToken(rr2, req2)

	if status := rr2.Code; status != http.StatusInternalServerError {
		t.Errorf("handler returned wrong status code for invalid key: got %v want %v", status, http.StatusInternalServerError)
	}
}
