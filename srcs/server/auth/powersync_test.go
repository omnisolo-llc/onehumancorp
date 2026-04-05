package auth_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestPowerSyncTokenGeneration(t *testing.T) {
	// 1. Initialize Store & Handlers
	store := auth.NewStore()
	handlers := auth.NewHandlers(store)

	// 2. Setup mock claims and request context
	claims := &auth.Claims{
		Subject:        "user-123",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
		IssuedAt:       time.Now().Unix(),
		Expires:        time.Now().Add(time.Hour).Unix(),
	}

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
	// Inject mock claims using ClaimsContextKeyForTest as per instructions/codebase pattern
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()

	// 3. Call the handler directly
	handlers.HandlePowerSyncToken(rr, req)

	// 4. Verify HTTP Response code
	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	// 5. Decode JSON response and verify fields
	var resp map[string]interface{}
	err := json.NewDecoder(rr.Body).Decode(&resp)
	if err != nil {
		t.Fatalf("failed to decode response JSON: %v", err)
	}

	token, ok := resp["token"].(string)
	if !ok || token == "" {
		t.Errorf("response did not contain valid token: %v", resp)
	}

	expFloat, ok := resp["expiresAt"].(float64)
	if !ok || expFloat == 0 {
		t.Errorf("response did not contain valid expiresAt: %v", resp)
	}
}

func TestPowerSyncJWKSEndpoint(t *testing.T) {
	store := auth.NewStore()
	handlers := auth.NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	rr := httptest.NewRecorder()

	handlers.HandlePowerSyncJWKS(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp struct {
		Keys []struct {
			Kty string `json:"kty"`
			Kid string `json:"kid"`
			Alg string `json:"alg"`
			Use string `json:"use"`
			Crv string `json:"crv"`
			X   string `json:"x"`
		} `json:"keys"`
	}

	err := json.NewDecoder(rr.Body).Decode(&resp)
	if err != nil {
		t.Fatalf("failed to decode jwks response JSON: %v", err)
	}

	if len(resp.Keys) != 1 {
		t.Fatalf("expected 1 key in JWKS, got %d", len(resp.Keys))
	}

	key := resp.Keys[0]
	if key.Kty != "OKP" || key.Kid != "powersync-key-1" || key.Alg != "EdDSA" || key.Use != "sig" || key.Crv != "Ed25519" || key.X == "" {
		t.Errorf("unexpected JWK fields: %+v", key)
	}
}
