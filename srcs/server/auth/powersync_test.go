package auth_test

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"context"
)

func TestPowerSyncHandlers_HandleToken(t *testing.T) {
	store := auth.NewStore()
	handlers := auth.NewPowerSyncAuthHandlers(store)

	req, _ := http.NewRequest("GET", "/api/auth/powersync/token", nil)
	rr := httptest.NewRecorder()

	// Call without auth claims
	handlers.HandleToken(rr, req)

	if rr.Code != http.StatusUnauthorized {
		t.Errorf("Expected status 401, got %v", rr.Code)
	}

	// Call with auth claims
	claims := &auth.Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{"user"},
	}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)
	rr = httptest.NewRecorder()

	handlers.HandleToken(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected status 200, got %v", rr.Code)
	}

	var resp map[string]interface{}
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}

	if _, ok := resp["token"]; !ok {
		t.Error("Expected token in response")
	}
}

func TestPowerSyncHandlers_HandleJWKS(t *testing.T) {
	store := auth.NewStore()
	handlers := auth.NewPowerSyncAuthHandlers(store)

	req, _ := http.NewRequest("GET", "/api/auth/powersync/jwks", nil)
	rr := httptest.NewRecorder()

	handlers.HandleJWKS(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected status 200, got %v", rr.Code)
	}

	var resp map[string]interface{}
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}

	keys, ok := resp["keys"].([]interface{})
	if !ok || len(keys) == 0 {
		t.Error("Expected keys in JWKS response")
	}
}
