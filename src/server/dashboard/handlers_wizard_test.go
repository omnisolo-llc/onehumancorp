package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/src/server/orchestration"
	"github.com/onehumancorp/mono/src/server/settings"
)

type mockSettingsStore struct{}

func (m mockSettingsStore) Update(cfg settings.AppSettings) error {
	return nil
}

func (m mockSettingsStore) Get() settings.AppSettings {
	return settings.AppSettings{}
}

func (m mockSettingsStore) Save() error {
	return nil
}

func (m mockSettingsStore) SetExtra(key, value string) error {
    return nil
}

func TestHandleWizardConfigure(t *testing.T) {
	s := &Server{
		settings: settings.AppSettings{},
		hub:      orchestration.NewHub(),
	}

	reqBody := wizardConfigureRequest{
		Extras: map[string]string{
			"company_name": "Test Company",
			"industry":     "Tech",
		},
	}
	body, _ := json.Marshal(reqBody)

	req, _ := http.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewBuffer(body))
	rr := httptest.NewRecorder()

	s.handleWizardConfigure(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	s.mu.RLock()
	defer s.mu.RUnlock()

	if s.settings.Extras["company_name"] != "Test Company" {
		t.Errorf("Expected company_name to be 'Test Company', got '%s'", s.settings.Extras["company_name"])
	}
}

// TestHandleWizardOnboardingVerify checks the onboarding verify endpoint
func TestHandleWizardOnboardingVerify(t *testing.T) {
	s := &Server{}

	// Test case 1: Method Not Allowed
	t.Run("Method Not Allowed", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/onboarding_verify", nil)
		rr := httptest.NewRecorder()
		s.handleWizardOnboardingVerify(rr, req)

		if rr.Code != http.StatusMethodNotAllowed {
			t.Errorf("Expected status 405, got %v", rr.Code)
		}
	})

	// Test case 2: Cloud mode missing both DATABASE_URL and REDIS_URL
	t.Run("Cloud missing env", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "false")
		t.Setenv("DATABASE_URL", "")
		t.Setenv("REDIS_URL", "")

		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/onboarding_verify", nil)
		rr := httptest.NewRecorder()
		s.handleWizardOnboardingVerify(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("Expected status 200, got %v", rr.Code)
		}

		var responseMap map[string]interface{}
		if err := json.Unmarshal(rr.Body.Bytes(), &responseMap); err != nil {
			t.Fatalf("Failed to parse response: %v", err)
		}

		if responseMap["status"] != "degraded" {
			t.Errorf("Expected status to be degraded, got %v", responseMap["status"])
		}
	})

	// Test case 3: Cloud mode missing DATABASE_URL only
	t.Run("Cloud missing db", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "false")
		t.Setenv("DATABASE_URL", "")
		t.Setenv("REDIS_URL", "redis://localhost:6379")

		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/onboarding_verify", nil)
		rr := httptest.NewRecorder()
		s.handleWizardOnboardingVerify(rr, req)

		var responseMap map[string]interface{}
		json.Unmarshal(rr.Body.Bytes(), &responseMap)

		if responseMap["status"] != "degraded" {
			t.Errorf("Expected status to be degraded, got %v", responseMap["status"])
		}
	})

	// Test case 4: Cloud mode missing REDIS_URL only
	t.Run("Cloud missing redis", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "false")
		t.Setenv("DATABASE_URL", "postgres://localhost:5432")
		t.Setenv("REDIS_URL", "")

		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/onboarding_verify", nil)
		rr := httptest.NewRecorder()
		s.handleWizardOnboardingVerify(rr, req)

		var responseMap map[string]interface{}
		json.Unmarshal(rr.Body.Bytes(), &responseMap)

		if responseMap["status"] != "degraded" {
			t.Errorf("Expected status to be degraded, got %v", responseMap["status"])
		}
	})

	// Test case 5: Cloud mode complete
	t.Run("Cloud complete", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "false")
		t.Setenv("DATABASE_URL", "postgres://localhost:5432")
		t.Setenv("REDIS_URL", "redis://localhost:6379")

		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/onboarding_verify", nil)
		rr := httptest.NewRecorder()
		s.handleWizardOnboardingVerify(rr, req)

		var responseMap map[string]interface{}
		json.Unmarshal(rr.Body.Bytes(), &responseMap)

		if responseMap["status"] != "healthy" {
			t.Errorf("Expected status to be healthy, got %v", responseMap["status"])
		}
	})

	// Test case 6: Standalone mode
	t.Run("Standalone", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "true")

		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/onboarding_verify", nil)
		rr := httptest.NewRecorder()
		s.handleWizardOnboardingVerify(rr, req)

		var responseMap map[string]interface{}
		json.Unmarshal(rr.Body.Bytes(), &responseMap)

		if responseMap["status"] != "healthy" {
			t.Errorf("Expected status to be healthy, got %v", responseMap["status"])
		}
		if responseMap["mode"] != "standalone" {
			t.Errorf("Expected mode to be standalone, got %v", responseMap["mode"])
		}
	})
}
