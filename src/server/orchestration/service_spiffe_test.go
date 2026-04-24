package orchestration

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestRequireSPIFFE(t *testing.T) {
	handler := requireSPIFFE(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	})

	tests := []struct {
		name       string
		authHeader string
		wantStatus int
	}{
		{
			name:       "missing header",
			authHeader: "",
			wantStatus: http.StatusUnauthorized,
		},
		{
			name:       "invalid format",
			authHeader: "Basic token",
			wantStatus: http.StatusUnauthorized,
		},
		{
			name:       "invalid spiffe id",
			authHeader: "Bearer invalid-id",
			wantStatus: http.StatusUnauthorized,
		},
		{
			name:       "valid spiffe id",
			authHeader: "Bearer spiffe://onehumancorp.io/agent/service",
			wantStatus: http.StatusOK,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest("GET", "/", nil)
			if tt.authHeader != "" {
				req.Header.Set("Authorization", tt.authHeader)
			}
			rr := httptest.NewRecorder()
			handler.ServeHTTP(rr, req)

			if rr.Code != tt.wantStatus {
				t.Errorf("expected status %d, got %d", tt.wantStatus, rr.Code)
			}
		})
	}
}
