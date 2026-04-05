package auth_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestPowerSyncTokenHandler(t *testing.T) {
	req := httptest.NewRequest("GET", "/api/auth/powersync/token", nil)

	// Inject mock claims
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Subject:        "user-123",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	})
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler := auth.PowerSyncTokenHandler()
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var response map[string]string
	if err := json.Unmarshal(rr.Body.Bytes(), &response); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if response["token"] == "" {
		t.Errorf("expected token in response, got none")
	}
}

func TestPowerSyncJWKSHandler(t *testing.T) {
	req := httptest.NewRequest("GET", "/api/auth/powersync/jwks", nil)
	rr := httptest.NewRecorder()
	handler := auth.PowerSyncJWKSHandler()
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var response map[string]interface{}
	if err := json.Unmarshal(rr.Body.Bytes(), &response); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	keys, ok := response["keys"].([]interface{})
	if !ok || len(keys) == 0 {
		t.Errorf("expected keys in JWKS response, got none")
	}
}
