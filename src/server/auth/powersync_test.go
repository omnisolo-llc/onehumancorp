package auth

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestPowerSyncTokenHandler(t *testing.T) {
	req := httptest.NewRequest("GET", "/api/auth/powersync/token", nil)

	// Inject mock claims
	claims := &Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	}
	ctx := context.WithValue(req.Context(), ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler := PowerSyncTokenHandler(NewStore())

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var response map[string]interface{}
	err := json.Unmarshal(rr.Body.Bytes(), &response)
	if err != nil {
		t.Fatalf("failed to unmarshal response: %v", err)
	}

	if response["token"] == "" {
		t.Errorf("expected token in response")
	}
}

func TestPowerSyncJWKSHandler(t *testing.T) {
	req := httptest.NewRequest("GET", "/api/auth/powersync/jwks", nil)
	rr := httptest.NewRecorder()

	handler := PowerSyncJWKSHandler()
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var response map[string]interface{}
	err := json.Unmarshal(rr.Body.Bytes(), &response)
	if err != nil {
		t.Fatalf("failed to unmarshal response: %v", err)
	}

	keys, ok := response["keys"].([]interface{})
	if !ok || len(keys) == 0 {
		t.Errorf("expected keys in JWKS response")
	}
}
