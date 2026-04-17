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
	s := &Server{
		settings: settings.AppSettings{},
		hub:      orchestration.NewHub(),
	}

	tests := []struct {
		name           string
		method         string
		envVars        map[string]string
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
			name:   "Standalone Mode",
			method: http.MethodGet,
			envVars: map[string]string{
				"OHC_STANDALONE": "true",
			},
			expectedStatus: http.StatusOK,
			expectedMode:   "standalone",
			expectedHealth: "healthy",
		},
		{
			name:   "Cloud Mode - Missing Database URL",
			method: http.MethodGet,
			envVars: map[string]string{
				"REDIS_URL": "redis://localhost:6379",
			},
			expectedStatus: http.StatusOK,
			expectedMode:   "cloud",
			expectedHealth: "degraded",
		},
		{
			name:   "Cloud Mode - Missing Redis URL",
			method: http.MethodGet,
			envVars: map[string]string{
				"DATABASE_URL": "postgres://localhost:5432",
			},
			expectedStatus: http.StatusOK,
			expectedMode:   "cloud",
			expectedHealth: "degraded",
		},
		{
			name:   "Cloud Mode - Both Missing",
			method: http.MethodGet,
			envVars: map[string]string{},
			expectedStatus: http.StatusOK,
			expectedMode:   "cloud",
			expectedHealth: "degraded",
		},
		{
			name:   "Cloud Mode - Valid URLs",
			method: http.MethodGet,
			envVars: map[string]string{
				"DATABASE_URL": "postgres://localhost:5432",
				"REDIS_URL":    "redis://localhost:6379",
			},
			expectedStatus: http.StatusOK,
			expectedMode:   "cloud",
			expectedHealth: "healthy",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Save current env and set new ones for test
			for k, v := range tt.envVars {
				t.Setenv(k, v)
			}

			req, _ := http.NewRequest(tt.method, "/api/wizard/onboarding/verify", nil)
			rr := httptest.NewRecorder()

			s.handleWizardOnboardingVerify(rr, req)

			if status := rr.Code; status != tt.expectedStatus {
				t.Errorf("handler returned wrong status code: got %v want %v", status, tt.expectedStatus)
			}

			if tt.expectedStatus == http.StatusOK {
				var resp map[string]interface{}
				if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
					t.Fatalf("Failed to parse JSON response: %v", err)
				}

				if resp["mode"] != tt.expectedMode {
					t.Errorf("Expected mode %s, got %s", tt.expectedMode, resp["mode"])
				}

				if resp["status"] != tt.expectedHealth {
					t.Errorf("Expected health status %s, got %s", tt.expectedHealth, resp["status"])
				}
			}
		})
	}
}
