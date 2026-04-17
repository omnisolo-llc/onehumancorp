package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
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

	tests := []struct {
		name           string
		method         string
		standalone     string
		dbURL          string
		redisURL       string
		expectedStatus int
		expectedMode   string
		expectedHealth string
	}{
		{
			name:           "Method Not Allowed",
			method:         http.MethodPost,
			expectedStatus: http.StatusMethodNotAllowed,
		},
		{
			name:           "Standalone Mode",
			method:         http.MethodGet,
			standalone:     "true",
			expectedStatus: http.StatusOK,
			expectedMode:   "standalone",
			expectedHealth: "healthy",
		},
		{
			name:           "Cloud Mode Healthy",
			method:         http.MethodGet,
			standalone:     "false",
			dbURL:          "postgres://user:pass@localhost:5432/db",
			redisURL:       "redis://localhost:6379",
			expectedStatus: http.StatusOK,
			expectedMode:   "cloud",
			expectedHealth: "healthy",
		},
		{
			name:           "Cloud Mode Missing DB",
			method:         http.MethodGet,
			standalone:     "false",
			dbURL:          "",
			redisURL:       "redis://localhost:6379",
			expectedStatus: http.StatusOK,
			expectedMode:   "cloud",
			expectedHealth: "degraded",
		},
		{
			name:           "Cloud Mode Missing Redis",
			method:         http.MethodGet,
			standalone:     "false",
			dbURL:          "postgres://user:pass@localhost:5432/db",
			redisURL:       "",
			expectedStatus: http.StatusOK,
			expectedMode:   "cloud",
			expectedHealth: "degraded",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			os.Setenv("OHC_STANDALONE", tt.standalone)
			os.Setenv("DATABASE_URL", tt.dbURL)
			os.Setenv("REDIS_URL", tt.redisURL)
			defer func() {
				os.Unsetenv("OHC_STANDALONE")
				os.Unsetenv("DATABASE_URL")
				os.Unsetenv("REDIS_URL")
			}()

			req := httptest.NewRequest(tt.method, "/api/wizard/onboarding_verify", nil)
			w := httptest.NewRecorder()

			s.handleWizardOnboardingVerify(w, req)

			if w.Code != tt.expectedStatus {
				t.Errorf("expected status %v, got %v", tt.expectedStatus, w.Code)
			}

			if tt.expectedStatus == http.StatusOK {
				var resp map[string]interface{}
				if err := json.NewDecoder(w.Body).Decode(&resp); err != nil {
					t.Fatalf("failed to decode response: %v", err)
				}

				if resp["mode"] != tt.expectedMode {
					t.Errorf("expected mode %v, got %v", tt.expectedMode, resp["mode"])
				}

				if resp["status"] != tt.expectedHealth {
					t.Errorf("expected status %v, got %v", tt.expectedHealth, resp["status"])
				}
			}
		})
	}
}
