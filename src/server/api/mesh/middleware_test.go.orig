package mesh

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestValidationMiddleware(t *testing.T) {
	tests := []struct {
		name           string
		method         string
		path           string
		body           string
		expectedStatus int
	}{
		{
			name:           "Valid Payload",
			method:         "POST",
			path:           "/api/mesh/broadcast",
			body:           `{"agent_id": "123", "channel": "mesh:tasks", "event_type": "TASK", "data": {"foo": "bar"}}`,
			expectedStatus: http.StatusOK,
		},
		{
			name:           "Missing agent_id",
			method:         "POST",
			path:           "/api/mesh/broadcast",
			body:           `{"channel": "mesh:tasks", "event_type": "TASK", "data": {"foo": "bar"}}`,
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Deprecated action key",
			method:         "POST",
			path:           "/api/mesh/broadcast",
			body:           `{"agent_id": "123", "channel": "mesh:tasks", "event_type": "TASK", "data": {"foo": "bar"}, "action": "do_something"}`,
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Empty agent_id string",
			method:         "POST",
			path:           "/api/mesh/broadcast",
			body:           `{"agent_id": "", "channel": "mesh:tasks", "event_type": "TASK", "data": {"foo": "bar"}}`,
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Ignore GET requests",
			method:         "GET",
			path:           "/api/mesh/broadcast",
			body:           ``,
			expectedStatus: http.StatusOK,
		},
		{
			name:           "Ignore other paths",
			method:         "POST",
			path:           "/api/other",
			body:           `{"foo": "bar"}`,
			expectedStatus: http.StatusOK,
		},
	}

	handler := ValidationMiddleware(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest(tt.method, tt.path, bytes.NewBufferString(tt.body))
			w := httptest.NewRecorder()
			handler.ServeHTTP(w, req)

			if w.Code != tt.expectedStatus {
				t.Errorf("Expected status %d, got %d", tt.expectedStatus, w.Code)
			}
		})
	}
}
