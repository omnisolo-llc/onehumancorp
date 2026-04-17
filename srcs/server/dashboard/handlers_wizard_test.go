package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/settings"
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

		var resp map[string]interface{}
		if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
			t.Fatalf("Failed to parse response: %v", err)
		}

		if resp["status"] != "degraded" {
			t.Errorf("Expected status to be degraded, got %v", resp["status"])
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

		var resp map[string]interface{}
		json.Unmarshal(rr.Body.Bytes(), &resp)

		if resp["status"] != "degraded" {
			t.Errorf("Expected status to be degraded, got %v", resp["status"])
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

		var resp map[string]interface{}
		json.Unmarshal(rr.Body.Bytes(), &resp)

		if resp["status"] != "degraded" {
			t.Errorf("Expected status to be degraded, got %v", resp["status"])
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

		var resp map[string]interface{}
		json.Unmarshal(rr.Body.Bytes(), &resp)

		if resp["status"] != "healthy" {
			t.Errorf("Expected status to be healthy, got %v", resp["status"])
		}
	})

	// Test case 6: Standalone mode
	t.Run("Standalone", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "true")

		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/onboarding_verify", nil)
		rr := httptest.NewRecorder()
		s.handleWizardOnboardingVerify(rr, req)

		var resp map[string]interface{}
		json.Unmarshal(rr.Body.Bytes(), &resp)

		if resp["status"] != "healthy" {
			t.Errorf("Expected status to be healthy, got %v", resp["status"])
		}
		if resp["mode"] != "standalone" {
			t.Errorf("Expected mode to be standalone, got %v", resp["mode"])
		}
	})
}






func TestHandleWizardStatus(t *testing.T) {
	t.Run("Method Not Allowed", func(t *testing.T) {
		s := &Server{
			settings: settings.AppSettings{},
			hub:      orchestration.NewHub(),
		}
		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/status", nil)
		rr := httptest.NewRecorder()
		s.handleWizardStatus(rr, req)

		if rr.Code != http.StatusMethodNotAllowed {
			t.Errorf("Expected status 405, got %v", rr.Code)
		}
	})

	t.Run("Not Configured", func(t *testing.T) {
		s := &Server{
			settings: settings.AppSettings{
				ListenAddr: "",
			},
			hub: orchestration.NewHub(),
		}
		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/status", nil)
		rr := httptest.NewRecorder()
		s.handleWizardStatus(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("Expected status 200, got %v", rr.Code)
		}
		var resp wizardStatusResponse
		json.Unmarshal(rr.Body.Bytes(), &resp)
		if resp.Configured {
			t.Errorf("Expected configured to be false")
		}
	})

	t.Run("Configured", func(t *testing.T) {
		s := &Server{
			settings: settings.AppSettings{
				ListenAddr:    ":8080",
				DBPath:        "/data/db",
				CentrifugeURL: "http://localhost:8000",
				AiProviders: []settings.AiProvider{
					{Enabled: true},
				},
			},
			hub: orchestration.NewHub(),
		}
		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/status", nil)
		rr := httptest.NewRecorder()
		s.handleWizardStatus(rr, req)

		var resp wizardStatusResponse
		json.Unmarshal(rr.Body.Bytes(), &resp)
		if !resp.Configured {
			t.Errorf("Expected configured to be true")
		}
	})
}


func TestHandleWizardConfigure(t *testing.T) {
	t.Run("Method Not Allowed", func(t *testing.T) {
		s := &Server{
			settings: settings.AppSettings{},
			hub:      orchestration.NewHub(),
		}

		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/configure", nil)
		rr := httptest.NewRecorder()
		s.handleWizardConfigure(rr, req)

		if rr.Code != http.StatusMethodNotAllowed {
			t.Errorf("Expected status 405, got %v", rr.Code)
		}
	})

	t.Run("Invalid JSON", func(t *testing.T) {
		s := &Server{
			settings: settings.AppSettings{},
			hub:      orchestration.NewHub(),
		}

		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewBufferString("{invalidjson}"))
		rr := httptest.NewRecorder()
		s.handleWizardConfigure(rr, req)

		if rr.Code != http.StatusBadRequest {
			t.Errorf("Expected status 400, got %v", rr.Code)
		}
	})

	t.Run("Valid Update", func(t *testing.T) {
		s := &Server{
			settings: settings.AppSettings{},
			hub:      orchestration.NewHub(),
		}

		reqBody := wizardConfigureRequest{
			ListenAddr:    ":8080",
			DBPath:        "/data/db",
			PostgresURL:   "postgres://localhost",
			RedisURL:      "redis://localhost",
			CentrifugeURL: "http://localhost",
			MinimaxAPIKey: "key123",
			Extras: map[string]string{
				"company_name": "Test Company",
				"industry":     "Tech",
			},
			AiProviders: []settings.AiProvider{
				{Enabled: true},
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
		if s.settings.ListenAddr != ":8080" {
			t.Errorf("Expected ListenAddr ':8080'")
		}
		if s.settings.DBPath != "/data/db" {
			t.Errorf("Expected DBPath '/data/db'")
		}
		if s.settings.PostgresURL != "postgres://localhost" {
			t.Errorf("Expected PostgresURL 'postgres://localhost'")
		}
		if s.settings.RedisURL != "redis://localhost" {
			t.Errorf("Expected RedisURL 'redis://localhost'")
		}
		if s.settings.CentrifugeURL != "http://localhost" {
			t.Errorf("Expected CentrifugeURL 'http://localhost'")
		}
		if s.settings.MinimaxAPIKey != "key123" {
			t.Errorf("Expected MinimaxAPIKey 'key123'")
		}
		if len(s.settings.AiProviders) == 0 {
			t.Errorf("Expected AiProviders to be set")
		}
	})

	t.Run("Update With Existing Extras", func(t *testing.T) {
		s := &Server{
			settings: settings.AppSettings{
				Extras: map[string]string{"existing": "value"},
			},
			hub: orchestration.NewHub(),
		}

		reqBody := wizardConfigureRequest{
			Extras: map[string]string{
				"new": "value2",
			},
		}
		body, _ := json.Marshal(reqBody)
		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewBuffer(body))
		rr := httptest.NewRecorder()

		s.handleWizardConfigure(rr, req)

		s.mu.RLock()
		defer s.mu.RUnlock()

		if s.settings.Extras["existing"] != "value" {
			t.Errorf("Expected existing extra to be preserved")
		}
		if s.settings.Extras["new"] != "value2" {
			t.Errorf("Expected new extra to be added")
		}
	})
}
