package auth_test

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandlePowerSyncToken(t *testing.T) {
	// Setup the environment
	seedBase64 := base64.RawURLEncoding.EncodeToString([]byte("test-powersync-key-32-bytes-long123"))
	os.Setenv("OHC_POWERSYNC_PRIV_KEY", seedBase64)
	defer os.Unsetenv("OHC_POWERSYNC_PRIV_KEY")

	store := auth.NewStore()
	handlers := auth.NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)

	// Inject mock claims
	claims := &auth.Claims{
		Subject:        "user123",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
		Expires:        time.Now().Add(1 * time.Hour).Unix(),
	}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	w := httptest.NewRecorder()
	handlers.HandlePowerSyncToken(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusOK {
		t.Fatalf("expected status OK, got %d", res.StatusCode)
	}

	var respBody map[string]string
	if err := json.NewDecoder(res.Body).Decode(&respBody); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if respBody["token"] == "" {
		t.Errorf("expected token in response, got empty")
	}
}

func TestHandlePowerSyncJWKS(t *testing.T) {
	seedBase64 := base64.RawURLEncoding.EncodeToString([]byte("test-powersync-key-32-bytes-long123"))
	os.Setenv("OHC_POWERSYNC_PRIV_KEY", seedBase64)
	defer os.Unsetenv("OHC_POWERSYNC_PRIV_KEY")

	store := auth.NewStore()
	handlers := auth.NewHandlers(store)

	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	w := httptest.NewRecorder()
	handlers.HandlePowerSyncJWKS(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusOK {
		t.Fatalf("expected status OK, got %d", res.StatusCode)
	}

	var respBody map[string]interface{}
	if err := json.NewDecoder(res.Body).Decode(&respBody); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	keys, ok := respBody["keys"].([]interface{})
	if !ok || len(keys) == 0 {
		t.Errorf("expected keys in response")
	}
}
