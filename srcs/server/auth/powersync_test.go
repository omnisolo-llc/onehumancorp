package auth

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandlePowerSyncToken(t *testing.T) {
	handlers := NewHandlers(nil) // Store not needed for this handler directly

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
	claims := &Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
	}
	ctx := context.WithValue(req.Context(), claimsContextKey, claims)
	req = req.WithContext(ctx)

	w := httptest.NewRecorder()
	handlers.HandlePowerSyncToken(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200 OK, got %d", w.Code)
	}

	var resp powerSyncTokenResponse
	if err := json.NewDecoder(w.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Token == "" {
		t.Error("expected non-empty token")
	}
}

func TestHandlePowerSyncJWKS(t *testing.T) {
	handlers := NewHandlers(nil)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	w := httptest.NewRecorder()

	handlers.HandlePowerSyncJWKS(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200 OK, got %d", w.Code)
	}

	var resp jwksResponse
	if err := json.NewDecoder(w.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode jwks response: %v", err)
	}

	if len(resp.Keys) != 1 {
		t.Fatalf("expected 1 key in jwks, got %d", len(resp.Keys))
	}

	key := resp.Keys[0]
	if key["kty"] != "OKP" || key["crv"] != "Ed25519" || key["use"] != "sig" {
		t.Errorf("unexpected key properties: %v", key)
	}
}
