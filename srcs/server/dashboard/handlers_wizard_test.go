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


func TestHandleWizardStatus(t *testing.T) {
	s := &Server{
		settings: settings.AppSettings{
			ListenAddr:    "127.0.0.1:8080",
			DBPath:        "/tmp/db",
			CentrifugeURL: "ws://localhost:8000",
			AiProviders: []settings.AiProvider{
				{Enabled: true},
			},
		},
		hub: orchestration.NewHub(),
	}

	t.Run("Method Not Allowed", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/status", nil)
		rr := httptest.NewRecorder()

		s.handleWizardStatus(rr, req)

		if rr.Code != http.StatusMethodNotAllowed {
			t.Errorf("handler returned wrong status code: got %v want %v", rr.Code, http.StatusMethodNotAllowed)
		}
	})

	t.Run("Status Configured", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodGet, "/api/wizard/status", nil)
		rr := httptest.NewRecorder()

		s.handleWizardStatus(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("handler returned wrong status code: got %v want %v", rr.Code, http.StatusOK)
		}

		var resp wizardStatusResponse
		if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
			t.Fatalf("Failed to parse response: %v", err)
		}

		if !resp.Configured {
			t.Errorf("Expected Configured to be true")
		}
		if !resp.Steps.Server {
			t.Errorf("Expected Steps.Server to be true")
		}
		if !resp.Steps.AiProvider {
			t.Errorf("Expected Steps.AiProvider to be true")
		}
		if !resp.Steps.Centrifuge {
			t.Errorf("Expected Steps.Centrifuge to be true")
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
			t.Errorf("handler returned wrong status code: got %v want %v", rr.Code, http.StatusMethodNotAllowed)
		}
	})

	t.Run("Invalid JSON", func(t *testing.T) {
		s := &Server{
			settings: settings.AppSettings{},
			hub:      orchestration.NewHub(),
		}

		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewBuffer([]byte(`{"unknown": "field"`)))
		rr := httptest.NewRecorder()

		s.handleWizardConfigure(rr, req)

		if rr.Code != http.StatusBadRequest {
			t.Errorf("handler returned wrong status code: got %v want %v", rr.Code, http.StatusBadRequest)
		}
	})

	t.Run("Valid Configuration", func(t *testing.T) {
		s_test := &Server{
			settings: settings.AppSettings{},
			hub:      orchestration.NewHub(),
		}
		store := settings.NewStore()
		s_test.hub.SetSettingsStore(store)

		reqBody := wizardConfigureRequest{
			ListenAddr:    "127.0.0.1:8080",
			DBPath:        "/tmp/db",
			PostgresURL:   "postgres://user:pass@localhost:5432/db",
			RedisURL:      "redis://localhost:6379",
			CentrifugeURL: "ws://localhost:8000",
			MinimaxAPIKey: "secret-key",
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

		s_test.handleWizardConfigure(rr, req)

		if status := rr.Code; status != http.StatusOK {
			t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
		}

		s_test.mu.RLock()
		defer s_test.mu.RUnlock()

		if s_test.settings.ListenAddr != "127.0.0.1:8080" {
			t.Errorf("Expected ListenAddr to be '127.0.0.1:8080', got '%s'", s_test.settings.ListenAddr)
		}
		if s_test.settings.DBPath != "/tmp/db" {
			t.Errorf("Expected DBPath to be '/tmp/db', got '%s'", s_test.settings.DBPath)
		}
		if s_test.settings.PostgresURL != "postgres://user:pass@localhost:5432/db" {
			t.Errorf("Expected PostgresURL to be 'postgres://user:pass@localhost:5432/db', got '%s'", s_test.settings.PostgresURL)
		}
		if s_test.settings.RedisURL != "redis://localhost:6379" {
			t.Errorf("Expected RedisURL to be 'redis://localhost:6379', got '%s'", s_test.settings.RedisURL)
		}
		if s_test.settings.CentrifugeURL != "ws://localhost:8000" {
			t.Errorf("Expected CentrifugeURL to be 'ws://localhost:8000', got '%s'", s_test.settings.CentrifugeURL)
		}
		if s_test.settings.MinimaxAPIKey != "secret-key" {
			t.Errorf("Expected MinimaxAPIKey to be 'secret-key', got '%s'", s_test.settings.MinimaxAPIKey)
		}
		if s_test.settings.Extras["company_name"] != "Test Company" {
			t.Errorf("Expected company_name to be 'Test Company', got '%s'", s_test.settings.Extras["company_name"])
		}
		if len(s_test.settings.AiProviders) != 1 || !s_test.settings.AiProviders[0].Enabled {
			t.Errorf("Expected 1 enabled AI Provider")
		}
	})

	t.Run("Update Existing Extras", func(t *testing.T) {
		s_test := &Server{
			settings: settings.AppSettings{
				Extras: map[string]string{
					"company_name": "Test Company",
				},
			},
			hub: orchestration.NewHub(),
		}
		store := settings.NewStore()
		s_test.hub.SetSettingsStore(store)

		reqBody := wizardConfigureRequest{
			Extras: map[string]string{
				"new_key": "new_value",
			},
		}
		body, _ := json.Marshal(reqBody)

		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewBuffer(body))
		rr := httptest.NewRecorder()

		s_test.handleWizardConfigure(rr, req)

		if status := rr.Code; status != http.StatusOK {
			t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
		}

		s_test.mu.RLock()
		defer s_test.mu.RUnlock()

		if s_test.settings.Extras["company_name"] != "Test Company" {
			t.Errorf("Expected company_name to be 'Test Company', got '%s'", s_test.settings.Extras["company_name"])
		}
		if s_test.settings.Extras["new_key"] != "new_value" {
			t.Errorf("Expected new_key to be 'new_value', got '%s'", s_test.settings.Extras["new_key"])
		}
	})

	t.Run("Nil Extras Map Allocation", func(t *testing.T) {
		s_test := &Server{
			settings: settings.AppSettings{},
			hub:      orchestration.NewHub(),
		}
		store := settings.NewStore()
		s_test.hub.SetSettingsStore(store)

		reqBody := wizardConfigureRequest{
			Extras: map[string]string{
				"key": "val",
			},
		}
		body, _ := json.Marshal(reqBody)

		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewBuffer(body))
		rr := httptest.NewRecorder()

		s_test.handleWizardConfigure(rr, req)

		if status := rr.Code; status != http.StatusOK {
			t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
		}

		s_test.mu.RLock()
		defer s_test.mu.RUnlock()
		if s_test.settings.Extras == nil {
			t.Errorf("Expected Extras to be initialized")
		}
	})

	t.Run("New AiProviders", func(t *testing.T) {
		s_test := &Server{
			settings: settings.AppSettings{},
			hub:      orchestration.NewHub(),
		}
		store := settings.NewStore()
		s_test.hub.SetSettingsStore(store)

		reqBody := wizardConfigureRequest{
			AiProviders: []settings.AiProvider{
				{Enabled: true, Name: "OpenAI"},
			},
		}
		body, _ := json.Marshal(reqBody)

		req, _ := http.NewRequest(http.MethodPost, "/api/wizard/configure", bytes.NewBuffer(body))
		rr := httptest.NewRecorder()

		s_test.handleWizardConfigure(rr, req)

		if status := rr.Code; status != http.StatusOK {
			t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
		}

		s_test.mu.RLock()
		defer s_test.mu.RUnlock()
		if len(s_test.settings.AiProviders) != 1 || s_test.settings.AiProviders[0].Name != "OpenAI" {
			t.Errorf("Expected AiProviders to be initialized")
		}
	})

	s := &Server{
		settings: settings.AppSettings{},
		hub:      orchestration.NewHub(),
	}
	store := settings.NewStore()
	s.hub.SetSettingsStore(store)

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

func TestHasEnabledProvider(t *testing.T) {
	t.Run("Has provider", func(t *testing.T) {
		providers := []settings.AiProvider{
			{Enabled: false},
			{Enabled: true},
		}
		if !hasEnabledProvider(providers) {
			t.Errorf("Expected hasEnabledProvider to return true")
		}
	})

	t.Run("No provider", func(t *testing.T) {
		providers := []settings.AiProvider{
			{Enabled: false},
		}
		if hasEnabledProvider(providers) {
			t.Errorf("Expected hasEnabledProvider to return false")
		}
	})

	t.Run("Empty", func(t *testing.T) {
		var providers []settings.AiProvider
		if hasEnabledProvider(providers) {
			t.Errorf("Expected hasEnabledProvider to return false for empty list")
		}
	})
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
