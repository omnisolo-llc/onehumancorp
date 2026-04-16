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


func TestDiagnosticsHandler(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/api/diagnostics", nil)
	rr := httptest.NewRecorder()

	DiagnosticsHandler(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var res map[string]string
	if err := json.NewDecoder(rr.Body).Decode(&res); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if res["status"] != "ok" {
		t.Errorf("handler returned unexpected status: got %v want ok", res["status"])
	}
}
