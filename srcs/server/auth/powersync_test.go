package auth_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandlePowerSyncToken(t *testing.T) {
	store := auth.NewStore()
	h := auth.NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)

	// Inject mock claims
	claims := &auth.Claims{
		Subject:        "user-123",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	w := httptest.NewRecorder()
	h.HandlePowerSyncToken(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}

	if _, ok := resp["token"]; !ok {
		t.Fatalf("expected token in response")
	}
}

func TestHandlePowerSyncJWKS(t *testing.T) {
	store := auth.NewStore()
	h := auth.NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	w := httptest.NewRecorder()
	h.HandlePowerSyncJWKS(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}

	var resp struct {
		Keys []map[string]interface{} `json:"keys"`
	}
	if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}

	if len(resp.Keys) == 0 {
		t.Fatalf("expected keys in response")
	}
}
