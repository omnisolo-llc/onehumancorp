package auth

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestPowerSyncJWKSHandler(t *testing.T) {
	store := NewStore()
	h := NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	rr := httptest.NewRecorder()

	h.PowerSyncJWKSHandler(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected 200 OK, got %d", rr.Code)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
		t.Fatalf("Failed to parse response: %v", err)
	}

	keys, ok := resp["keys"].([]interface{})
	if !ok || len(keys) == 0 {
		t.Errorf("Expected 'keys' array in response")
	}
}

func TestPowerSyncTokenHandler(t *testing.T) {
	store := NewStore()
	h := NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)

	claims := &Claims{
		Subject: "user123",
		OrganizationID: "org-1",
	}
	ctx := context.WithValue(req.Context(), ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()

	h.PowerSyncTokenHandler(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected 200 OK, got %d", rr.Code)
	}

	var resp map[string]string
	if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
		t.Fatalf("Failed to parse response: %v", err)
	}

	token, ok := resp["token"]
	if !ok || token == "" {
		t.Errorf("Expected token in response")
	}
}

func TestPowerSyncTokenHandler_Unauthorized(t *testing.T) {
	store := NewStore()
	h := NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
	rr := httptest.NewRecorder()

	h.PowerSyncTokenHandler(rr, req)

	if rr.Code != http.StatusUnauthorized {
		t.Errorf("Expected 401 Unauthorized, got %d", rr.Code)
	}
}
