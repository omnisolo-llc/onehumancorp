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
	s := &Server{
		settings: settings.AppSettings{
			ListenAddr:    "127.0.0.1:8080",
			DBPath:        "/tmp/db",
			CentrifugeURL: "ws://localhost:8000",
			AiProviders: []settings.AiProvider{
				{Enabled: true, Name: "minimax"},
			},
		},
		hub: orchestration.NewHub(),
	}

	// Test case 1: Method Not Allowed
	t.Run("Method Not Allowed", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/status", nil)
		rr := httptest.NewRecorder()
		s.handleWizardStatus(rr, req)

		if rr.Code != http.StatusMethodNotAllowed {
			t.Errorf("Expected status 405, got %v", rr.Code)
		}
	})

	// Test case 2: Valid status
	t.Run("Valid Status", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/status", nil)
		rr := httptest.NewRecorder()
		s.handleWizardStatus(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("Expected status 200, got %v", rr.Code)
		}

		var resp wizardStatusResponse
		if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
			t.Fatalf("Failed to parse response: %v", err)
		}

		if !resp.Configured {
			t.Errorf("Expected configured to be true, got %v", resp.Configured)
		}
		if !resp.Steps.Server {
			t.Errorf("Expected server step to be true")
		}
		if !resp.Steps.AiProvider {
			t.Errorf("Expected AiProvider step to be true")
		}
		if !resp.Steps.Centrifuge {
			t.Errorf("Expected Centrifuge step to be true")
		}
	})
}

func TestHasEnabledProvider(t *testing.T) {
	t.Run("No providers", func(t *testing.T) {
		if hasEnabledProvider(nil) {
			t.Error("Expected false for nil providers")
		}
		if hasEnabledProvider([]settings.AiProvider{}) {
			t.Error("Expected false for empty providers")
		}
	})

	t.Run("None enabled", func(t *testing.T) {
		providers := []settings.AiProvider{
			{Enabled: false, Name: "minimax"},
			{Enabled: false, Name: "openai"},
		}
		if hasEnabledProvider(providers) {
			t.Error("Expected false when all are disabled")
		}
	})

	t.Run("One enabled", func(t *testing.T) {
		providers := []settings.AiProvider{
			{Enabled: false, Name: "minimax"},
			{Enabled: true, Name: "openai"},
		}
		if !hasEnabledProvider(providers) {
			t.Error("Expected true when one is enabled")
		}
	})
}

func TestHandleWizardConfigure_FullCoverage(t *testing.T) {
	s := &Server{
		settings: settings.AppSettings{},
		hub:      orchestration.NewHub(),
	}

	t.Run("Method Not Allowed", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/configure", nil)
		rr := httptest.NewRecorder()
		s.handleWizardConfigure(rr, req)

		if rr.Code != http.StatusMethodNotAllowed {
			t.Errorf("Expected status 405, got %v", rr.Code)
		}
	})

	t.Run("Invalid JSON", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewBuffer([]byte("{invalid json}")))
		rr := httptest.NewRecorder()
		s.handleWizardConfigure(rr, req)

		if rr.Code != http.StatusBadRequest {
			t.Errorf("Expected status 400, got %v", rr.Code)
		}
	})

	t.Run("All Fields Configured", func(t *testing.T) {
		reqBody := wizardConfigureRequest{
			ListenAddr:    "127.0.0.1:9090",
			DBPath:        "/new/db/path",
			PostgresURL:   "postgres://user:pass@localhost:5432/db",
			RedisURL:      "redis://localhost:6379/1",
			CentrifugeURL: "ws://localhost:8000/connection",
			MinimaxAPIKey: "test-minimax-key",
			AiProviders: []settings.AiProvider{
				{Enabled: true, Name: "openai"},
			},
		}
		body, _ := json.Marshal(reqBody)

		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewBuffer(body))
		rr := httptest.NewRecorder()

		s.handleWizardConfigure(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("Expected status 200, got %v", rr.Code)
		}

		s.mu.RLock()
		defer s.mu.RUnlock()

		if s.settings.ListenAddr != "127.0.0.1:9090" {
			t.Errorf("Expected ListenAddr to be set")
		}
		if s.settings.DBPath != "/new/db/path" {
			t.Errorf("Expected DBPath to be set")
		}
		if s.settings.PostgresURL != "postgres://user:pass@localhost:5432/db" {
			t.Errorf("Expected PostgresURL to be set")
		}
		if s.settings.RedisURL != "redis://localhost:6379/1" {
			t.Errorf("Expected RedisURL to be set")
		}
		if s.settings.CentrifugeURL != "ws://localhost:8000/connection" {
			t.Errorf("Expected CentrifugeURL to be set")
		}
		if s.settings.MinimaxAPIKey != "test-minimax-key" {
			t.Errorf("Expected MinimaxAPIKey to be set")
		}
		if len(s.settings.AiProviders) == 0 || s.settings.AiProviders[0].Name != "openai" {
			t.Errorf("Expected AiProviders to be set")
		}
	})
}
