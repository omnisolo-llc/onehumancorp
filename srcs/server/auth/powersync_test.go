package auth

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandlePowerSyncToken(t *testing.T) {
	s := NewStore()
	h := NewHandlers(s)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)

	claims := &Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{RoleViewer},
	}
	ctx := context.WithValue(req.Context(), ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	w := httptest.NewRecorder()
	h.HandlePowerSyncToken(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200 OK, got %v", w.Code)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}

	if _, ok := resp["token"]; !ok {
		t.Errorf("expected token in response")
	}
}

func TestHandlePowerSyncJWKS(t *testing.T) {
	s := NewStore()
	h := NewHandlers(s)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	w := httptest.NewRecorder()
	h.HandlePowerSyncJWKS(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200 OK, got %v", w.Code)
	}

	var resp powersyncJWKSResponse
	if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}

	if len(resp.Keys) == 0 {
		t.Errorf("expected keys in response")
	}
	if resp.Keys[0].Kid != "powersync-key-1" {
		t.Errorf("expected kid to be powersync-key-1")
	}
}
