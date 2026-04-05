package auth_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandlePowerSyncToken_Success(t *testing.T) {
	s := auth.NewStore()
	h := auth.NewHandlers(s)

	os.Setenv("PS_JWT_SECRET", "test-secret")
	defer os.Unsetenv("PS_JWT_SECRET")

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)

	// Inject mock claims
	mockClaims := &auth.Claims{
		Subject:        "test-sub",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, mockClaims)
	req = req.WithContext(ctx)

	rec := httptest.NewRecorder()
	h.HandlePowerSyncToken(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d. Body: %s", rec.Code, rec.Body.String())
	}

	var resp map[string]string
	if err := json.NewDecoder(rec.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	token, ok := resp["token"]
	if !ok || token == "" {
		t.Fatal("response missing token")
	}

	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		t.Fatalf("invalid token format, expected 3 parts, got %d", len(parts))
	}
}

func TestHandlePowerSyncToken_Unauthorized(t *testing.T) {
	s := auth.NewStore()
	h := auth.NewHandlers(s)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
	rec := httptest.NewRecorder()

	h.HandlePowerSyncToken(rec, req)

	if rec.Code != http.StatusUnauthorized {
		t.Fatalf("expected 401, got %d", rec.Code)
	}
}

func TestHandlePowerSyncToken_MethodNotAllowed(t *testing.T) {
	s := auth.NewStore()
	h := auth.NewHandlers(s)

	req := httptest.NewRequest(http.MethodPost, "/api/auth/powersync/token", nil)
	rec := httptest.NewRecorder()

	h.HandlePowerSyncToken(rec, req)

	if rec.Code != http.StatusMethodNotAllowed {
		t.Fatalf("expected 405, got %d", rec.Code)
	}
}
