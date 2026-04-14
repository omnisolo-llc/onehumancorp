package main

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/services/onboarding"
)

func TestRunCLI_Success(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		resp := onboarding.VerificationResponse{
			Status: "healthy",
			Mode:   "standalone",
			Diagnostics: []onboarding.Diagnostic{
				{Check: "TEST", Status: "ok", Message: "All good"},
			},
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}))
	defer server.Close()

	if err := RunCLI(server.URL); err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
}
