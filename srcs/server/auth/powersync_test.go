package auth_test

import (
	"context"
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func setupPowerSyncEnv(t *testing.T) ed25519.PrivateKey {
	_, priv, err := ed25519.GenerateKey(rand.Reader)
	if err != nil {
		t.Fatalf("failed to generate ed25519 key: %v", err)
	}

	// OHC_POWERSYNC_PRIV_KEY must be the 32-byte seed for standard ed25519 lib.
	seed := priv.Seed()
	seedBase64 := base64.RawURLEncoding.EncodeToString(seed)
	t.Setenv("OHC_POWERSYNC_PRIV_KEY", seedBase64)

	return priv
}

func TestHandlePowerSyncToken_Success(t *testing.T) {
	setupPowerSyncEnv(t)
	s := auth.NewStore()
	h := auth.NewHandlers(s)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
	claims := &auth.Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rec := httptest.NewRecorder()
	h.HandlePowerSyncToken(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200 OK, got %d: %s", rec.Code, rec.Body.String())
	}

	var resp map[string]string
	if err := json.NewDecoder(rec.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	token := resp["token"]
	if token == "" {
		t.Fatal("expected token to be present")
	}

	// Basic check of the token structure (3 parts)
	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		t.Fatalf("expected 3 parts in token, got %d", len(parts))
	}
}

func TestHandlePowerSyncToken_Unauthenticated(t *testing.T) {
	setupPowerSyncEnv(t)
	s := auth.NewStore()
	h := auth.NewHandlers(s)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
	rec := httptest.NewRecorder()
	h.HandlePowerSyncToken(rec, req)

	if rec.Code != http.StatusUnauthorized {
		t.Fatalf("expected 401 Unauthorized, got %d", rec.Code)
	}
}

func TestHandlePowerSyncToken_MissingKey(t *testing.T) {
	t.Setenv("OHC_POWERSYNC_PRIV_KEY", "")
	s := auth.NewStore()
	h := auth.NewHandlers(s)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
	claims := &auth.Claims{Subject: "user-1"}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rec := httptest.NewRecorder()
	h.HandlePowerSyncToken(rec, req)

	if rec.Code != http.StatusInternalServerError {
		t.Fatalf("expected 500 Internal Server Error, got %d", rec.Code)
	}
}

func TestHandlePowerSyncJWKS_Success(t *testing.T) {
	setupPowerSyncEnv(t)
	s := auth.NewStore()
	h := auth.NewHandlers(s)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	rec := httptest.NewRecorder()
	h.HandlePowerSyncJWKS(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200 OK, got %d: %s", rec.Code, rec.Body.String())
	}

	var resp struct {
		Keys []struct {
			Kty string `json:"kty"`
			Crv string `json:"crv"`
			X   string `json:"x"`
			Kid string `json:"kid"`
			Use string `json:"use"`
		} `json:"keys"`
	}
	if err := json.NewDecoder(rec.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(resp.Keys) != 1 {
		t.Fatalf("expected 1 key, got %d", len(resp.Keys))
	}

	key := resp.Keys[0]
	if key.Kty != "OKP" || key.Crv != "Ed25519" || key.Use != "sig" || key.Kid != "ohc-powersync-key-1" {
		t.Errorf("unexpected key values: %+v", key)
	}
	if key.X == "" {
		t.Error("expected non-empty public key X value")
	}
}
