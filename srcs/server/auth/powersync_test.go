package auth

import (
	"context"
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestPowerSyncEndpoints(t *testing.T) {
	// Generate a test key
	pub, priv, err := ed25519.GenerateKey(nil)
	if err != nil {
		t.Fatalf("Failed to generate test key: %v", err)
	}

	seed := priv.Seed()
	seedBase64 := base64.RawURLEncoding.EncodeToString(seed)
	os.Setenv("OHC_POWERSYNC_PRIV_KEY", seedBase64)
	defer os.Unsetenv("OHC_POWERSYNC_PRIV_KEY")

	// Test Token Endpoint
	req := httptest.NewRequest("GET", "/api/auth/powersync/token", nil)
	w := httptest.NewRecorder()

	// Inject mock claims
	ctx := context.WithValue(req.Context(), ClaimsContextKeyForTest, &Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	})
	req = req.WithContext(ctx)

	HandlePowerSyncToken(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("Expected status 200, got %d. Body: %s", w.Code, w.Body.String())
	}

	var tokenResp PowerSyncTokenResponse
	if err := json.Unmarshal(w.Body.Bytes(), &tokenResp); err != nil {
		t.Fatalf("Failed to parse token response: %v", err)
	}

	if tokenResp.Token == "" {
		t.Errorf("Expected a token, got empty string")
	}

	// Test JWKS Endpoint
	reqJWKS := httptest.NewRequest("GET", "/api/auth/powersync/jwks", nil)
	wJWKS := httptest.NewRecorder()

	HandlePowerSyncJWKS(wJWKS, reqJWKS)

	if wJWKS.Code != http.StatusOK {
		t.Fatalf("Expected status 200, got %d. Body: %s", wJWKS.Code, wJWKS.Body.String())
	}

	var jwksResp PowerSyncJWKSResponse
	if err := json.Unmarshal(wJWKS.Body.Bytes(), &jwksResp); err != nil {
		t.Fatalf("Failed to parse JWKS response: %v", err)
	}

	if len(jwksResp.Keys) == 0 {
		t.Fatalf("Expected at least one key in JWKS, got 0")
	}

	expectedX := base64.RawURLEncoding.EncodeToString(pub)
	if jwksResp.Keys[0].X != expectedX {
		t.Errorf("Expected pub key %s, got %s", expectedX, jwksResp.Keys[0].X)
	}
}

func TestPowerSyncTokenWithoutClaims(t *testing.T) {
	req := httptest.NewRequest("GET", "/api/auth/powersync/token", nil)
	w := httptest.NewRecorder()

	HandlePowerSyncToken(w, req)

	if w.Code != http.StatusUnauthorized {
		t.Fatalf("Expected status 401, got %d", w.Code)
	}
}
