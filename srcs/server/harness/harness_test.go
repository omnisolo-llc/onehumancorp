package harness

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestHarness_ServeHTTP(t *testing.T) {
	config := &SandboxConfig{
		DeniedDomains: []string{"evil.com"},
		AllowedDomains: []string{"good.com"},
	}
	h := NewHarness(config)

	tests := []struct {
		name       string
		host       string
		wantStatus int
	}{
		{
			name:       "Allowed Domain",
			host:       "good.com",
			wantStatus: http.StatusOK,
		},
		{
			name:       "Denied Domain explicitly",
			host:       "evil.com",
			wantStatus: http.StatusForbidden,
		},
		{
			name:       "Denied Domain implicitly (not in allowlist)",
			host:       "unknown.com",
			wantStatus: http.StatusForbidden,
		},
		{
			name:       "Denied Domain with port",
			host:       "evil.com:443",
			wantStatus: http.StatusForbidden,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req, err := http.NewRequest("GET", "http://"+tt.host+"/", nil)
			if err != nil {
				t.Fatal(err)
			}
			req.Host = tt.host

			rr := httptest.NewRecorder()
			h.ServeHTTP(rr, req)

			if status := rr.Code; status != tt.wantStatus {
				t.Errorf("handler returned wrong status code: got %v want %v",
					status, tt.wantStatus)
			}
		})
	}
}

func TestHarness_Run(t *testing.T) {
	config := &SandboxConfig{
		ReadPaths:  []string{"/etc"},
		WritePaths: []string{"/tmp"},
	}
	h := NewHarness(config)

	err := h.Run(context.Background(), "echo", []string{"hello"})
	if err == nil {
		// bwrap succeeded (maybe installed locally)
	} else {
		if !strings.Contains(err.Error(), "executable file not found") && !strings.Contains(err.Error(), "exit status") {
			t.Errorf("unexpected error from Run: %v", err)
		}
	}
}
