package orchestration

import (
	"encoding/base64"
	"encoding/json"
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
		wantCode   int
	}{
		{
			name:       "Missing header",
			authHeader: "",
			wantCode:   http.StatusUnauthorized,
		},
		{
			name:       "Invalid prefix",
			authHeader: "Basic something",
			wantCode:   http.StatusUnauthorized,
		},
		{
			name:       "Not a JWT",
			authHeader: "Bearer spiffe://something",
			wantCode:   http.StatusUnauthorized,
		},
		{
			name:       "Invalid JWT payload",
			authHeader: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.invalid.signature",
			wantCode:   http.StatusUnauthorized,
		},
		{
			name: "Valid JWT with valid SPIFFE ID",
			authHeader: func() string {
				payload := map[string]string{"sub": "spiffe://onehumancorp.io/agent/123"}
				payloadBytes, _ := json.Marshal(payload)
				encodedPayload := base64.RawURLEncoding.EncodeToString(payloadBytes)
				return "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9." + encodedPayload + ".signature"
			}(),
			wantCode: http.StatusOK,
		},
		{
			name: "Valid JWT with invalid SPIFFE ID",
			authHeader: func() string {
				payload := map[string]string{"sub": "spiffe://invalid.domain/agent/123"}
				payloadBytes, _ := json.Marshal(payload)
				encodedPayload := base64.RawURLEncoding.EncodeToString(payloadBytes)
				return "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9." + encodedPayload + ".signature"
			}(),
			wantCode: http.StatusForbidden,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest("GET", "/", nil)
			if tt.authHeader != "" {
				req.Header.Set("Authorization", tt.authHeader)
			}
			rec := httptest.NewRecorder()
			handler(rec, req)

			if rec.Code != tt.wantCode {
				t.Errorf("expected code %d, got %d", tt.wantCode, rec.Code)
			}
		})
	}
}
