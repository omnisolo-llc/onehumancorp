import re

with open("srcs/server/dashboard/handlers_wizard_test.go", "a") as f:
    f.write("""
func TestHandleWizardStatus(t *testing.T) {
	s := &Server{
		settings: settings.AppSettings{
			ListenAddr:    "127.0.0.1:8080",
			DBPath:        "/tmp/db",
			CentrifugeURL: "http://localhost:8000",
			AiProviders: []settings.AiProvider{
				{Enabled: true},
			},
		},
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

	// Test case 2: Success
	t.Run("Success", func(t *testing.T) {
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
			t.Errorf("Expected Server step to be true")
		}
		if !resp.Steps.AiProvider {
			t.Errorf("Expected AiProvider step to be true")
		}
		if !resp.Steps.Centrifuge {
			t.Errorf("Expected Centrifuge step to be true")
		}
	})
}
""")
