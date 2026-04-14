package onboarding

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestHandleStatusGet(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	req, err := http.NewRequest(http.MethodGet, "/api/onboarding/status", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleStatus)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	var respStatus OnboardingStatus
	if err := json.NewDecoder(rr.Body).Decode(&respStatus); err != nil {
		t.Fatal(err)
	}

	if respStatus.Mode != "Standalone Desktop" {
		t.Errorf("Expected Mode Standalone Desktop, got %v", respStatus.Mode)
	}
}

func TestHandleStatusPost(t *testing.T) {
	req, err := http.NewRequest(http.MethodPost, "/api/onboarding/status", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleStatus)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusMethodNotAllowed)
	}
}
