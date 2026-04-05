package auth

import (
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestHandlePowerSyncJWKS(t *testing.T) {
	// Generate a random seed
	seed := make([]byte, ed25519.SeedSize)
	_, err := rand.Read(seed)
	if err != nil {
		t.Fatalf("failed to generate seed: %v", err)
	}

	privKeyStr := base64.RawURLEncoding.EncodeToString(seed)

	os.Setenv("OHC_POWERSYNC_PRIV_KEY", privKeyStr)
	defer os.Unsetenv("OHC_POWERSYNC_PRIV_KEY")

	h := NewHandlers(nil)

	req, err := http.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	if err != nil {
		t.Fatalf("failed to create request: %v", err)
	}

	rr := httptest.NewRecorder()
	h.HandlePowerSyncJWKS(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
		t.Logf("Response body: %s", rr.Body.String())
	}

	var response struct {
		Keys []map[string]interface{} `json:"keys"`
	}
	if err := json.Unmarshal(rr.Body.Bytes(), &response); err != nil {
		t.Fatalf("failed to unmarshal response: %v", err)
	}

	if len(response.Keys) == 0 {
		t.Errorf("expected keys in response, got none")
	}

	if response.Keys[0]["kty"] != "OKP" {
		t.Errorf("expected kty OKP, got %v", response.Keys[0]["kty"])
	}

	if response.Keys[0]["crv"] != "Ed25519" {
		t.Errorf("expected crv Ed25519, got %v", response.Keys[0]["crv"])
	}

	if response.Keys[0]["x"] == "" {
		t.Errorf("expected x in response, got empty")
	}
}
