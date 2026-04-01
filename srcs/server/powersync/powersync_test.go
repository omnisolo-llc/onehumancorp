package powersync

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestPowerSyncTokenHandler(t *testing.T) {
	// Initialize keys (normally done in init())
	if rsaPrivateKey == nil {
		err := loadOrGenerateKeys()
		if err != nil {
			t.Fatalf("failed to load/generate keys: %v", err)
		}
	}

	req := httptest.NewRequest("GET", "/api/powersync/token", nil)

	// Add claims to context
	claims := &auth.Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
	}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler := PowerSyncTokenHandler()
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var response map[string]string
	if err := json.NewDecoder(rr.Body).Decode(&response); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if response["token"] == "" {
		t.Errorf("expected token in response, got empty")
	}
}

func TestJWKSHandler(t *testing.T) {
	if rsaPublicKey == nil {
		err := loadOrGenerateKeys()
		if err != nil {
			t.Fatalf("failed to load/generate keys: %v", err)
		}
	}

	req := httptest.NewRequest("GET", "/.well-known/jwks.json", nil)
	rr := httptest.NewRecorder()
	handler := JWKSHandler()
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var response map[string]interface{}
	if err := json.NewDecoder(rr.Body).Decode(&response); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	keys, ok := response["keys"].([]interface{})
	if !ok || len(keys) == 0 {
		t.Fatalf("expected keys in response")
	}

	key := keys[0].(map[string]interface{})
	if key["kid"] != keyID {
		t.Errorf("expected kid %v, got %v", keyID, key["kid"])
	}
	if key["alg"] != "RS256" {
		t.Errorf("expected alg RS256, got %v", key["alg"])
	}
}
