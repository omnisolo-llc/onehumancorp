package auth

import (
	"context"
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestHandlePowerSyncToken(t *testing.T) {
	// Generate a random seed
	seed := make([]byte, ed25519.SeedSize)
	_, err := rand.Read(seed)
	if err != nil {
		t.Fatalf("failed to generate seed: %v", err)
	}

	privKeyStr := base64.RawURLEncoding.EncodeToString(seed)

	os.Setenv("OHC_POWERSYNC_PRIV_KEY", privKeyStr)
	os.Setenv("POWERSYNC_URL", "http://powersync:8080")
	defer os.Unsetenv("OHC_POWERSYNC_PRIV_KEY")
	defer os.Unsetenv("POWERSYNC_URL")

	h := NewHandlers(nil)

	req, err := http.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
	if err != nil {
		t.Fatalf("failed to create request: %v", err)
	}

	// Mock claims
	claims := &Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{RoleAdmin},
	}
	ctx := context.WithValue(req.Context(), ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	h.HandlePowerSyncToken(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
		t.Logf("Response body: %s", rr.Body.String())
	}

	var response map[string]string
	if err := json.Unmarshal(rr.Body.Bytes(), &response); err != nil {
		t.Fatalf("failed to unmarshal response: %v", err)
	}

	if response["token"] == "" {
		t.Errorf("expected token in response, got empty")
	}

	if response["powersync_url"] != "http://powersync:8080" {
		t.Errorf("expected powersync_url in response, got %s", response["powersync_url"])
	}
}
