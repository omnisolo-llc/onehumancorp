package onboarding

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestProvisionHandler(t *testing.T) {
	reqBody, _ := json.Marshal(ProvisionRequest{})
	req := httptest.NewRequest(http.MethodPost, "/api/provision", bytes.NewReader(reqBody))
	req.Header.Set("Content-Type", "application/json")

	rr := httptest.NewRecorder()
	ProvisionHandler(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var res map[string]string
	json.NewDecoder(rr.Body).Decode(&res)
	if res["status"] != "provisioned" {
		t.Errorf("handler returned unexpected body: got %v", res)
	}
}


func TestGenerateConfigHandler(t *testing.T) {
	tests := []struct {
		name       string
		mode       string
		wantStatus int
		wantDB     string
	}{
		{"cloud mode", "cloud", http.StatusOK, "postgresql"},
		{"standalone mode", "standalone", http.StatusOK, "sqlite"},
		{"thin client mode", "thin_client", http.StatusOK, "remote"},
		{"invalid mode", "invalid", http.StatusBadRequest, ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			reqBody, _ := json.Marshal(map[string]string{"mode": tt.mode})
			req := httptest.NewRequest(http.MethodPost, "/api/generate-config", bytes.NewReader(reqBody))
			req.Header.Set("Content-Type", "application/json")

			rr := httptest.NewRecorder()
			GenerateConfigHandler(rr, req)

			if status := rr.Code; status != tt.wantStatus {
				t.Errorf("handler returned wrong status code: got %v want %v", status, tt.wantStatus)
			}

			if tt.wantStatus == http.StatusOK {
				var res map[string]interface{}
				json.NewDecoder(rr.Body).Decode(&res)
				if res["status"] != "success" {
					t.Errorf("handler returned unexpected status: got %v", res["status"])
				}
				config := res["config"].(map[string]interface{})
				if config["database"] != tt.wantDB {
					t.Errorf("handler returned unexpected database: got %v want %v", config["database"], tt.wantDB)
				}
			}
		})
	}
}

func TestVerifyEnvironmentHandler(t *testing.T) {
    tests := []struct {
        name       string
        env        map[string]string
        wantStatus int
    }{
        {
            name: "valid standalone",
            env: map[string]string{
                "OHC_SOURCE_MODE": "standalone",
            },
            wantStatus: http.StatusOK,
        },
        {
            name: "invalid cloud",
            env: map[string]string{
                "OHC_SOURCE_MODE": "cloud",
                "OHC_MULTITENANT": "false",
            },
            wantStatus: http.StatusBadRequest,
        },
        {
            name: "valid thin client",
            env: map[string]string{
                "OHC_SOURCE_MODE": "thin_client",
                "OHC_API_ENDPOINT": "https://api.ohc.io",
            },
            wantStatus: http.StatusOK,
        },
        {
            name: "invalid thin client missing endpoint",
            env: map[string]string{
                "OHC_SOURCE_MODE": "thin_client",
            },
            wantStatus: http.StatusBadRequest,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            for k, v := range tt.env {
                t.Setenv(k, v)
            }

            req := httptest.NewRequest(http.MethodGet, "/api/verify-environment", nil)
            rr := httptest.NewRecorder()
            VerifyEnvironmentHandler(rr, req)

            if status := rr.Code; status != tt.wantStatus {
                t.Errorf("handler returned wrong status code: got %v want %v", status, tt.wantStatus)
            }
        })
    }
}

func TestWizardStateHandler(t *testing.T) {
    reqBody, _ := json.Marshal(map[string]interface{}{"step": 2, "name": "TestCorp"})
    req := httptest.NewRequest(http.MethodPost, "/api/wizard/state/save", bytes.NewReader(reqBody))
    req.Header.Set("Content-Type", "application/json")
    rr := httptest.NewRecorder()
    SaveWizardStateHandler(rr, req)

    if status := rr.Code; status != http.StatusOK {
        t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
    }

    reqGet := httptest.NewRequest(http.MethodGet, "/api/wizard/state", nil)
    rrGet := httptest.NewRecorder()
    GetWizardStateHandler(rrGet, reqGet)

    if status := rrGet.Code; status != http.StatusOK {
        t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
    }

    var res map[string]interface{}
    json.NewDecoder(rrGet.Body).Decode(&res)
    if res["name"] != "TestCorp" {
        t.Errorf("handler returned unexpected body: got %v", res)
    }
}

func TestAuditSetupHandler(t *testing.T) {
    tests := []struct {
        name       string
        env        map[string]string
        wantStatus int
    }{
        {
            name: "valid standalone",
            env: map[string]string{
                "OHC_SOURCE_MODE": "standalone",
            },
            wantStatus: http.StatusOK,
        },
        {
            name: "invalid cloud",
            env: map[string]string{
                "OHC_SOURCE_MODE": "cloud",
                "OHC_MULTITENANT": "false",
            },
            wantStatus: http.StatusBadRequest,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            for k, v := range tt.env {
                t.Setenv(k, v)
            }

            reqBody, _ := json.Marshal(map[string]interface{}{"env": tt.env})
            req := httptest.NewRequest(http.MethodPost, "/api/audit-setup", bytes.NewReader(reqBody))
            req.Header.Set("Content-Type", "application/json")
            rr := httptest.NewRecorder()
            AuditSetupHandler(rr, req)

            if status := rr.Code; status != tt.wantStatus {
                t.Errorf("handler returned wrong status code: got %v want %v", status, tt.wantStatus)
            }
        })
    }
}

func TestDiagnosticsHandler(t *testing.T) {
	t.Setenv("OHC_SOURCE_MODE", "standalone")

	wizardMu.Lock()
	wizardState = map[string]interface{}{"step": float64(3), "status": "completed"}
	wizardMu.Unlock()

	req := httptest.NewRequest(http.MethodGet, "/api/diagnostics", nil)
	rr := httptest.NewRecorder()
	DiagnosticsHandler(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var res DiagnosticsResponse
	if err := json.NewDecoder(rr.Body).Decode(&res); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if res.Status != "success" {
		t.Errorf("expected status 'success', got '%v'", res.Status)
	}
	if res.Config == nil || res.Config.Mode != "standalone" {
		t.Errorf("expected config mode 'standalone', got '%v'", res.Config)
	}
	if res.Wizard["step"] != float64(3) {
		t.Errorf("expected wizard step 3, got '%v'", res.Wizard["step"])
	}
}

func TestResetWizardStateHandler(t *testing.T) {
	reqSaveBody, _ := json.Marshal(map[string]interface{}{"step": 5, "name": "ResetCorp"})
	reqSave := httptest.NewRequest(http.MethodPost, "/api/wizard/state/save", bytes.NewReader(reqSaveBody))
	reqSave.Header.Set("Content-Type", "application/json")
	rrSave := httptest.NewRecorder()
	SaveWizardStateHandler(rrSave, reqSave)

	if status := rrSave.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	reqReset := httptest.NewRequest(http.MethodPost, "/api/wizard/state/reset", nil)
	rrReset := httptest.NewRecorder()
	ResetWizardStateHandler(rrReset, reqReset)

	if status := rrReset.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	reqGet := httptest.NewRequest(http.MethodGet, "/api/wizard/state", nil)
	rrGet := httptest.NewRecorder()
	GetWizardStateHandler(rrGet, reqGet)

	if status := rrGet.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var res map[string]interface{}
	json.NewDecoder(rrGet.Body).Decode(&res)
	if len(res) != 0 {
		t.Errorf("expected empty state, got %v", res)
	}
}

func TestGetWizardProfileHandler_Cloud(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/api/wizard/profile?mode=cloud", nil)
	w := httptest.NewRecorder()

	GetWizardProfileHandler(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusOK {
		t.Errorf("expected status OK, got %v", res.StatusCode)
	}

	var profile EnvConfig
	if err := json.NewDecoder(res.Body).Decode(&profile); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if profile.Mode != "cloud" || !profile.MultiTenant || profile.DatabaseURL == "" {
		t.Errorf("unexpected cloud profile: %+v", profile)
	}
}

func TestGetWizardProfileHandler_Standalone(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/api/wizard/profile?mode=standalone", nil)
	w := httptest.NewRecorder()

	GetWizardProfileHandler(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusOK {
		t.Errorf("expected status OK, got %v", res.StatusCode)
	}

	var profile EnvConfig
	if err := json.NewDecoder(res.Body).Decode(&profile); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if profile.Mode != "standalone" || profile.MultiTenant || profile.DatabaseURL != "sqlite://local.db" {
		t.Errorf("unexpected standalone profile: %+v", profile)
	}
}

func TestGetWizardProfileHandler_InvalidMode(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/api/wizard/profile?mode=unknown", nil)
	w := httptest.NewRecorder()

	GetWizardProfileHandler(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusBadRequest {
		t.Errorf("expected status BadRequest, got %v", res.StatusCode)
	}
}
