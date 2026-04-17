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
		envVars        map[string]string
		expectedStatus int
		expectedBody   map[string]interface{}
	}{
		{
			name:   "method not allowed",
			method: http.MethodPost,
			expectedStatus: http.StatusMethodNotAllowed,
		},
		{
			name:   "standalone mode",
			method: http.MethodGet,
			envVars: map[string]string{
				"OHC_STANDALONE": "true",
			},
			expectedStatus: http.StatusOK,
			expectedBody: map[string]interface{}{
				"status": "healthy",
				"mode":   "standalone",
				"diagnostics": []interface{}{
					map[string]interface{}{
						"check":   "OHC_STANDALONE",
						"status":  "ok",
						"message": "Standalone mode active",
					},
				},
			},
		},
		{
			name:   "cloud mode healthy",
			method: http.MethodGet,
			envVars: map[string]string{
				"DATABASE_URL": "postgres://localhost",
				"REDIS_URL":    "redis://localhost",
			},
			expectedStatus: http.StatusOK,
			expectedBody: map[string]interface{}{
				"status": "healthy",
				"mode":   "cloud",
				"diagnostics": []interface{}{
					map[string]interface{}{
						"check":   "DATABASE_URL",
						"status":  "ok",
						"message": "DATABASE_URL is configured",
					},
					map[string]interface{}{
						"check":   "REDIS_URL",
						"status":  "ok",
						"message": "REDIS_URL is configured",
					},
				},
			},
		},
		{
			name:   "cloud mode missing both",
			method: http.MethodGet,
			envVars: map[string]string{
				"DATABASE_URL": "",
				"REDIS_URL":    "",
			},
			expectedStatus: http.StatusOK,
			expectedBody: map[string]interface{}{
				"status": "degraded",
				"mode":   "cloud",
				"diagnostics": []interface{}{
					map[string]interface{}{
						"check":   "DATABASE_URL",
						"status":  "missing",
						"message": "DATABASE_URL is required in cloud mode",
					},
					map[string]interface{}{
						"check":   "REDIS_URL",
						"status":  "missing",
						"message": "REDIS_URL is required in cloud mode",
					},
				},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			for k, v := range tt.envVars {
				if v == "" {
					os.Unsetenv(k)
				} else {
					os.Setenv(k, v)
				}
			}
			defer func() {
				for k := range tt.envVars {
					os.Unsetenv(k)
				}
			}()

			req, _ := http.NewRequest(tt.method, "/api/wizard/onboarding_verify", nil)
			rr := httptest.NewRecorder()

			s.handleWizardOnboardingVerify(rr, req)

			if status := rr.Code; status != tt.expectedStatus {
				t.Errorf("handler returned wrong status code: got %v want %v", status, tt.expectedStatus)
			}

			if tt.expectedStatus == http.StatusOK {
				var respBody map[string]interface{}
				if err := json.Unmarshal(rr.Body.Bytes(), &respBody); err != nil {
					t.Fatalf("failed to unmarshal response: %v", err)
				}

				if respBody["status"] != tt.expectedBody["status"] {
					t.Errorf("expected status %v, got %v", tt.expectedBody["status"], respBody["status"])
				}

				if respBody["mode"] != tt.expectedBody["mode"] {
					t.Errorf("expected mode %v, got %v", tt.expectedBody["mode"], respBody["mode"])
				}

                expectedDiagBytes, _ := json.Marshal(tt.expectedBody["diagnostics"])
                actualDiagBytes, _ := json.Marshal(respBody["diagnostics"])

				if string(expectedDiagBytes) != string(actualDiagBytes) {
					t.Errorf("expected diagnostics %s, got %s", expectedDiagBytes, actualDiagBytes)
				}
			}
		})
	}
}
