package onboarding

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestVerifyEnvironment_Success(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/wizard/onboarding_verify" {
			t.Errorf("expected path /api/wizard/onboarding_verify, got %s", r.URL.Path)
		}
		resp := VerificationResponse{
			Status: "healthy",
			Mode:   "standalone",
			Diagnostics: []Diagnostic{
				{Check: "OHC_STANDALONE", Status: "ok", Message: "Standalone mode active"},
			},
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}))
	defer server.Close()

	resp, err := VerifyEnvironment(server.URL)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp.Status != "healthy" {
		t.Errorf("expected status healthy, got %s", resp.Status)
	}
	if len(resp.Diagnostics) != 1 {
		t.Fatalf("expected 1 diagnostic, got %d", len(resp.Diagnostics))
	}
	if resp.Diagnostics[0].Check != "OHC_STANDALONE" {
		t.Errorf("expected check OHC_STANDALONE, got %s", resp.Diagnostics[0].Check)
	}
}
